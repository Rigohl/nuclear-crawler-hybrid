#!/usr/bin/env python3
"""
🔄 Model Checkpoint Management & Recovery System
Handles backup, restoration, and validation of ML model checkpoints
"""

import os
import sys
import json
import shutil
import hashlib
import tarfile
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Tuple
import argparse

class CheckpointManager:
    """Manages ML model checkpoints with backup and recovery capabilities"""
    
    def __init__(self, checkpoint_dir: str = "ffi/chapel", backup_dir: str = "backups/checkpoints"):
        self.checkpoint_dir = Path(checkpoint_dir)
        self.backup_dir = Path(backup_dir)
        self.backup_dir.mkdir(parents=True, exist_ok=True)
        
        # Checkpoint file patterns
        self.checkpoint_patterns = [
            "*.checkpoint",
            "*.model",
            "*.weights",
            "*.bin",
            "*.pt",
            "*.safetensors",
            "*.pkl",
        ]
        
        # Metadata file
        self.metadata_file = self.backup_dir / "checkpoint_metadata.json"
        self.load_metadata()
    
    def load_metadata(self):
        """Load checkpoint metadata"""
        if self.metadata_file.exists():
            with open(self.metadata_file, 'r') as f:
                self.metadata = json.load(f)
        else:
            self.metadata = {
                "checkpoints": {},
                "backups": [],
                "last_backup": None
            }
    
    def save_metadata(self):
        """Save checkpoint metadata"""
        with open(self.metadata_file, 'w') as f:
            json.dump(self.metadata, f, indent=2)
    
    def compute_checksum(self, file_path: Path) -> str:
        """Compute SHA256 checksum of a file"""
        sha256 = hashlib.sha256()
        with open(file_path, 'rb') as f:
            for chunk in iter(lambda: f.read(4096), b""):
                sha256.update(chunk)
        return sha256.hexdigest()
    
    def find_checkpoints(self) -> List[Path]:
        """Find all checkpoint files"""
        checkpoints = []
        for pattern in self.checkpoint_patterns:
            checkpoints.extend(self.checkpoint_dir.rglob(pattern))
        return sorted(set(checkpoints))
    
    def validate_checkpoint(self, checkpoint_path: Path) -> Tuple[bool, str]:
        """Validate a checkpoint file"""
        if not checkpoint_path.exists():
            return False, "File does not exist"
        
        if checkpoint_path.stat().st_size == 0:
            return False, "File is empty"
        
        # Check if file is readable
        try:
            with open(checkpoint_path, 'rb') as f:
                f.read(1024)  # Read first KB
        except Exception as e:
            return False, f"File is not readable: {e}"
        
        # Compute and verify checksum if available
        checkpoint_id = str(checkpoint_path.relative_to(self.checkpoint_dir))
        if checkpoint_id in self.metadata["checkpoints"]:
            stored_checksum = self.metadata["checkpoints"][checkpoint_id].get("checksum")
            if stored_checksum:
                current_checksum = self.compute_checksum(checkpoint_path)
                if current_checksum != stored_checksum:
                    return False, f"Checksum mismatch (corruption detected)"
        
        return True, "Valid"
    
    def backup_checkpoints(self, incremental: bool = False) -> Dict:
        """Create backup of all checkpoints"""
        print("🔄 Starting checkpoint backup...")
        
        checkpoints = self.find_checkpoints()
        
        if not checkpoints:
            print("ℹ️  No checkpoints found to backup")
            return {"status": "no_checkpoints", "count": 0}
        
        print(f"📦 Found {len(checkpoints)} checkpoint files")
        
        # Create backup directory with timestamp
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        backup_name = f"checkpoint_backup_{timestamp}"
        backup_path = self.backup_dir / backup_name
        backup_path.mkdir(parents=True, exist_ok=True)
        
        # Copy checkpoints
        backed_up = []
        skipped = []
        
        for checkpoint in checkpoints:
            try:
                # Validate before backup
                is_valid, message = self.validate_checkpoint(checkpoint)
                if not is_valid:
                    print(f"⚠️  Skipping invalid checkpoint {checkpoint.name}: {message}")
                    skipped.append(str(checkpoint))
                    continue
                
                # Compute checksum
                checksum = self.compute_checksum(checkpoint)
                
                # Check if incremental backup is enabled and file hasn't changed
                checkpoint_id = str(checkpoint.relative_to(self.checkpoint_dir))
                if incremental and checkpoint_id in self.metadata["checkpoints"]:
                    stored_checksum = self.metadata["checkpoints"][checkpoint_id].get("checksum")
                    if stored_checksum == checksum:
                        print(f"⏭️  Skipping unchanged: {checkpoint.name}")
                        skipped.append(str(checkpoint))
                        continue
                
                # Copy file
                relative_path = checkpoint.relative_to(self.checkpoint_dir)
                dest_path = backup_path / relative_path
                dest_path.parent.mkdir(parents=True, exist_ok=True)
                
                shutil.copy2(checkpoint, dest_path)
                
                # Update metadata
                self.metadata["checkpoints"][checkpoint_id] = {
                    "checksum": checksum,
                    "size": checkpoint.stat().st_size,
                    "last_backup": timestamp,
                    "path": str(checkpoint)
                }
                
                backed_up.append(str(checkpoint))
                print(f"✅ Backed up: {checkpoint.name}")
                
            except Exception as e:
                print(f"❌ Error backing up {checkpoint.name}: {e}")
                skipped.append(str(checkpoint))
        
        # Create tarball
        tarball_path = self.backup_dir / f"{backup_name}.tar.gz"
        print(f"📦 Creating tarball: {tarball_path.name}")
        
        with tarfile.open(tarball_path, "w:gz") as tar:
            tar.add(backup_path, arcname=backup_name)
        
        # Remove temporary directory
        shutil.rmtree(backup_path)
        
        # Update backup metadata
        backup_info = {
            "timestamp": timestamp,
            "tarball": str(tarball_path),
            "size": tarball_path.stat().st_size,
            "checkpoints_count": len(backed_up),
            "checkpoints": backed_up
        }
        
        self.metadata["backups"].append(backup_info)
        self.metadata["last_backup"] = timestamp
        self.save_metadata()
        
        print(f"\n✅ Backup complete!")
        print(f"   Files backed up: {len(backed_up)}")
        print(f"   Files skipped: {len(skipped)}")
        print(f"   Tarball: {tarball_path}")
        print(f"   Size: {tarball_path.stat().st_size / (1024*1024):.2f} MB")
        
        return {
            "status": "success",
            "timestamp": timestamp,
            "tarball": str(tarball_path),
            "backed_up": backed_up,
            "skipped": skipped
        }
    
    def restore_checkpoint(self, backup_timestamp: Optional[str] = None, 
                          checkpoint_name: Optional[str] = None) -> Dict:
        """Restore checkpoints from backup"""
        print("🔄 Starting checkpoint restoration...")
        
        # Find backup to restore
        if backup_timestamp:
            backup_info = next(
                (b for b in self.metadata["backups"] if b["timestamp"] == backup_timestamp),
                None
            )
        else:
            # Use latest backup
            if not self.metadata["backups"]:
                print("❌ No backups available")
                return {"status": "no_backups"}
            backup_info = self.metadata["backups"][-1]
            backup_timestamp = backup_info["timestamp"]
        
        if not backup_info:
            print(f"❌ Backup not found: {backup_timestamp}")
            return {"status": "backup_not_found"}
        
        tarball_path = Path(backup_info["tarball"])
        
        if not tarball_path.exists():
            print(f"❌ Backup tarball not found: {tarball_path}")
            return {"status": "tarball_not_found"}
        
        print(f"📦 Restoring from: {tarball_path.name}")
        
        # Extract tarball
        temp_dir = self.backup_dir / f"restore_temp_{backup_timestamp}"
        temp_dir.mkdir(parents=True, exist_ok=True)
        
        try:
            with tarfile.open(tarball_path, "r:gz") as tar:
                tar.extractall(temp_dir)
            
            # Find extracted directory
            extracted_dir = temp_dir / f"checkpoint_backup_{backup_timestamp}"
            
            if not extracted_dir.exists():
                print(f"❌ Extracted directory not found")
                return {"status": "extraction_failed"}
            
            # Restore checkpoints
            restored = []
            
            for checkpoint in extracted_dir.rglob("*"):
                if checkpoint.is_file():
                    # Determine destination path
                    relative_path = checkpoint.relative_to(extracted_dir)
                    dest_path = self.checkpoint_dir / relative_path
                    
                    # Skip if specific checkpoint requested and this isn't it
                    if checkpoint_name and checkpoint.name != checkpoint_name:
                        continue
                    
                    # Create parent directories
                    dest_path.parent.mkdir(parents=True, exist_ok=True)
                    
                    # Backup existing file if it exists
                    if dest_path.exists():
                        backup_existing = dest_path.with_suffix(dest_path.suffix + ".bak")
                        shutil.move(dest_path, backup_existing)
                        print(f"📦 Backed up existing: {dest_path.name} -> {backup_existing.name}")
                    
                    # Copy restored file
                    shutil.copy2(checkpoint, dest_path)
                    restored.append(str(relative_path))
                    print(f"✅ Restored: {relative_path}")
            
            print(f"\n✅ Restoration complete!")
            print(f"   Files restored: {len(restored)}")
            
            return {
                "status": "success",
                "timestamp": backup_timestamp,
                "restored": restored
            }
            
        except Exception as e:
            print(f"❌ Error during restoration: {e}")
            return {"status": "error", "message": str(e)}
        
        finally:
            # Cleanup temp directory
            if temp_dir.exists():
                shutil.rmtree(temp_dir)
    
    def validate_all_checkpoints(self) -> Dict:
        """Validate all checkpoints"""
        print("🔍 Validating all checkpoints...")
        
        checkpoints = self.find_checkpoints()
        
        if not checkpoints:
            print("ℹ️  No checkpoints found")
            return {"status": "no_checkpoints", "valid": [], "invalid": []}
        
        valid = []
        invalid = []
        
        for checkpoint in checkpoints:
            is_valid, message = self.validate_checkpoint(checkpoint)
            checkpoint_id = str(checkpoint.relative_to(self.checkpoint_dir))
            
            if is_valid:
                valid.append(checkpoint_id)
                print(f"✅ Valid: {checkpoint.name}")
            else:
                invalid.append({
                    "file": checkpoint_id,
                    "reason": message
                })
                print(f"❌ Invalid: {checkpoint.name} - {message}")
        
        print(f"\n📊 Validation Summary:")
        print(f"   Valid: {len(valid)}")
        print(f"   Invalid: {len(invalid)}")
        
        return {
            "status": "complete",
            "total": len(checkpoints),
            "valid": valid,
            "invalid": invalid
        }
    
    def list_backups(self) -> List[Dict]:
        """List all available backups"""
        print("📋 Available Backups:")
        print("=" * 70)
        
        if not self.metadata["backups"]:
            print("ℹ️  No backups found")
            return []
        
        for backup in sorted(self.metadata["backups"], key=lambda x: x["timestamp"], reverse=True):
            timestamp = backup["timestamp"]
            size_mb = backup["size"] / (1024 * 1024)
            count = backup["checkpoints_count"]
            
            print(f"\n📦 Backup: {timestamp}")
            print(f"   Files: {count}")
            print(f"   Size: {size_mb:.2f} MB")
            print(f"   Location: {backup['tarball']}")
        
        return self.metadata["backups"]
    
    def cleanup_old_backups(self, keep_last_n: int = 5) -> Dict:
        """Remove old backups, keeping only the last N"""
        print(f"🧹 Cleaning up old backups (keeping last {keep_last_n})...")
        
        if len(self.metadata["backups"]) <= keep_last_n:
            print(f"ℹ️  Only {len(self.metadata['backups'])} backups exist, no cleanup needed")
            return {"status": "no_cleanup_needed", "removed": []}
        
        # Sort by timestamp
        sorted_backups = sorted(self.metadata["backups"], key=lambda x: x["timestamp"], reverse=True)
        
        # Keep last N, remove others
        to_keep = sorted_backups[:keep_last_n]
        to_remove = sorted_backups[keep_last_n:]
        
        removed = []
        for backup in to_remove:
            tarball_path = Path(backup["tarball"])
            if tarball_path.exists():
                tarball_path.unlink()
                removed.append(backup["timestamp"])
                print(f"🗑️  Removed: {backup['timestamp']}")
        
        # Update metadata
        self.metadata["backups"] = to_keep
        self.save_metadata()
        
        print(f"✅ Cleanup complete! Removed {len(removed)} old backups")
        
        return {
            "status": "success",
            "removed": removed,
            "kept": [b["timestamp"] for b in to_keep]
        }


def main():
    parser = argparse.ArgumentParser(
        description="Model Checkpoint Management & Recovery System"
    )
    
    subparsers = parser.add_subparsers(dest="command", help="Command to execute")
    
    # Backup command
    backup_parser = subparsers.add_parser("backup", help="Create checkpoint backup")
    backup_parser.add_argument("--incremental", action="store_true",
                              help="Only backup changed files")
    
    # Restore command
    restore_parser = subparsers.add_parser("restore", help="Restore checkpoints from backup")
    restore_parser.add_argument("--timestamp", help="Backup timestamp to restore from")
    restore_parser.add_argument("--checkpoint", help="Specific checkpoint file to restore")
    
    # Validate command
    validate_parser = subparsers.add_parser("validate", help="Validate all checkpoints")
    
    # List command
    list_parser = subparsers.add_parser("list", help="List all backups")
    
    # Cleanup command
    cleanup_parser = subparsers.add_parser("cleanup", help="Remove old backups")
    cleanup_parser.add_argument("--keep", type=int, default=5,
                               help="Number of recent backups to keep (default: 5)")
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        sys.exit(1)
    
    # Initialize checkpoint manager
    manager = CheckpointManager()
    
    # Execute command
    if args.command == "backup":
        result = manager.backup_checkpoints(incremental=args.incremental)
    elif args.command == "restore":
        result = manager.restore_checkpoint(
            backup_timestamp=args.timestamp,
            checkpoint_name=args.checkpoint
        )
    elif args.command == "validate":
        result = manager.validate_all_checkpoints()
    elif args.command == "list":
        result = manager.list_backups()
    elif args.command == "cleanup":
        result = manager.cleanup_old_backups(keep_last_n=args.keep)
    else:
        parser.print_help()
        sys.exit(1)
    
    # Return appropriate exit code
    if isinstance(result, dict) and result.get("status") in ["success", "complete", "no_checkpoints", "no_cleanup_needed"]:
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main()
