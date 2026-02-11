#!/usr/bin/env python3
"""
🔁 Universal Retry System with State Persistence
Provides comprehensive retry logic with exponential backoff, state tracking, and failure recovery
"""

import os
import sys
import json
import time
import subprocess
import hashlib
from pathlib import Path
from typing import Dict, List, Callable, Any, Optional, Tuple
from datetime import datetime, timedelta
import argparse

class RetryState:
    """Manages persistent retry state across runs"""
    
    def __init__(self, state_file: str = ".retry_state.json"):
        self.state_file = Path(state_file)
        self.state = self.load_state()
    
    def load_state(self) -> Dict:
        """Load retry state from file"""
        if self.state_file.exists():
            try:
                with open(self.state_file, 'r') as f:
                    return json.load(f)
            except Exception:
                pass
        return {
            "operations": {},
            "failures": {},
            "successes": {},
            "last_run": None
        }
    
    def save_state(self):
        """Save retry state to file"""
        try:
            with open(self.state_file, 'w') as f:
                json.dump(self.state, f, indent=2)
        except Exception as e:
            print(f"⚠️ Warning: Could not save state: {e}")
    
    def record_attempt(self, operation_id: str, success: bool, error: str = None):
        """Record an operation attempt"""
        if operation_id not in self.state["operations"]:
            self.state["operations"][operation_id] = {
                "attempts": 0,
                "first_attempt": datetime.now().isoformat(),
                "last_attempt": None,
                "success_count": 0,
                "failure_count": 0,
                "errors": []
            }
        
        op = self.state["operations"][operation_id]
        op["attempts"] += 1
        op["last_attempt"] = datetime.now().isoformat()
        
        if success:
            op["success_count"] += 1
            self.state["successes"][operation_id] = {
                "timestamp": datetime.now().isoformat(),
                "attempts_needed": op["attempts"]
            }
        else:
            op["failure_count"] += 1
            if error:
                op["errors"].append({
                    "timestamp": datetime.now().isoformat(),
                    "error": error
                })
            self.state["failures"][operation_id] = {
                "timestamp": datetime.now().isoformat(),
                "error": error
            }
        
        self.save_state()
    
    def get_operation_info(self, operation_id: str) -> Dict:
        """Get info about an operation"""
        return self.state["operations"].get(operation_id, {})
    
    def should_retry(self, operation_id: str, max_attempts: int = 10) -> bool:
        """Determine if operation should be retried"""
        op_info = self.get_operation_info(operation_id)
        return op_info.get("attempts", 0) < max_attempts
    
    def get_backoff_time(self, operation_id: str, base_delay: float = 1.0, max_delay: float = 300.0) -> float:
        """Calculate exponential backoff time"""
        op_info = self.get_operation_info(operation_id)
        attempts = op_info.get("attempts", 0)
        
        # Exponential backoff: base_delay * 2^attempts
        delay = min(base_delay * (2 ** attempts), max_delay)
        
        # Add jitter (±20%)
        import random
        jitter = delay * 0.2 * (random.random() * 2 - 1)
        
        return delay + jitter
    
    def reset_operation(self, operation_id: str):
        """Reset operation state"""
        if operation_id in self.state["operations"]:
            del self.state["operations"][operation_id]
        if operation_id in self.state["failures"]:
            del self.state["failures"][operation_id]
        self.save_state()
    
    def get_summary(self) -> Dict:
        """Get summary of all operations"""
        total_ops = len(self.state["operations"])
        total_successes = len(self.state["successes"])
        total_failures = len([op for op in self.state["operations"].values() if op["failure_count"] > 0])
        
        return {
            "total_operations": total_ops,
            "successful_operations": total_successes,
            "failed_operations": total_failures,
            "success_rate": total_successes / total_ops if total_ops > 0 else 0,
            "operations": self.state["operations"]
        }


class UniversalRetrySystem:
    """Universal retry system with persistence and comprehensive error handling"""
    
    def __init__(self, state_file: str = ".retry_state.json", max_attempts: int = 10):
        self.retry_state = RetryState(state_file)
        self.max_attempts = max_attempts
        
    def generate_operation_id(self, operation_name: str, *args) -> str:
        """Generate unique ID for an operation"""
        # Create hash of operation name and args
        content = f"{operation_name}:{str(args)}"
        return hashlib.md5(content.encode()).hexdigest()[:16]
    
    def execute_with_retry(
        self,
        operation_name: str,
        operation_func: Callable,
        *args,
        **kwargs
    ) -> Tuple[bool, Any, str]:
        """
        Execute an operation with automatic retry and state persistence
        
        Returns: (success, result, error_message)
        """
        operation_id = self.generate_operation_id(operation_name, *args)
        
        print(f"\n{'='*70}")
        print(f"🔁 Executing: {operation_name}")
        print(f"   Operation ID: {operation_id}")
        
        op_info = self.retry_state.get_operation_info(operation_id)
        if op_info:
            print(f"   Previous attempts: {op_info.get('attempts', 0)}")
            print(f"   Success count: {op_info.get('success_count', 0)}")
            print(f"   Failure count: {op_info.get('failure_count', 0)}")
        
        attempt = 0
        last_error = None
        
        while self.retry_state.should_retry(operation_id, self.max_attempts):
            attempt += 1
            op_info = self.retry_state.get_operation_info(operation_id)
            total_attempts = op_info.get("attempts", 0) + 1
            
            print(f"\n   📍 Attempt {total_attempts}/{self.max_attempts}")
            
            try:
                # Execute the operation
                result = operation_func(*args, **kwargs)
                
                # Record success
                self.retry_state.record_attempt(operation_id, True)
                
                print(f"   ✅ Success!")
                print(f"{'='*70}\n")
                
                return True, result, None
                
            except Exception as e:
                last_error = str(e)
                print(f"   ❌ Failed: {last_error}")
                
                # Record failure
                self.retry_state.record_attempt(operation_id, False, last_error)
                
                # Calculate backoff time
                if self.retry_state.should_retry(operation_id, self.max_attempts):
                    backoff = self.retry_state.get_backoff_time(operation_id)
                    print(f"   ⏰ Waiting {backoff:.2f}s before retry...")
                    time.sleep(backoff)
                else:
                    print(f"   🛑 Max attempts reached ({self.max_attempts})")
                    break
        
        print(f"{'='*70}\n")
        return False, None, last_error
    
    def execute_command_with_retry(
        self,
        command: List[str],
        operation_name: str = None,
        timeout: int = 300,
        cwd: str = None
    ) -> Tuple[bool, str, str]:
        """Execute a shell command with retry"""
        
        if not operation_name:
            operation_name = " ".join(command)
        
        def run_command():
            result = subprocess.run(
                command,
                capture_output=True,
                text=True,
                timeout=timeout,
                cwd=cwd
            )
            
            if result.returncode != 0:
                raise Exception(f"Command failed with exit code {result.returncode}: {result.stderr}")
            
            return result.stdout
        
        success, stdout, error = self.execute_with_retry(
            operation_name,
            run_command
        )
        
        return success, stdout or "", error or ""
    
    def get_persistent_summary(self) -> Dict:
        """Get summary of persistent state"""
        return self.retry_state.get_summary()
    
    def cleanup_old_operations(self, days: int = 30):
        """Clean up operations older than specified days"""
        cutoff = datetime.now() - timedelta(days=days)
        cutoff_str = cutoff.isoformat()
        
        operations_to_remove = []
        
        for op_id, op_info in self.retry_state.state["operations"].items():
            last_attempt = op_info.get("last_attempt", op_info.get("first_attempt"))
            if last_attempt and last_attempt < cutoff_str:
                operations_to_remove.append(op_id)
        
        for op_id in operations_to_remove:
            self.retry_state.reset_operation(op_id)
        
        print(f"🧹 Cleaned up {len(operations_to_remove)} old operations")
        return len(operations_to_remove)


# Pre-defined retry operations for common tasks
class CommonRetryOperations:
    """Common operations that benefit from retry logic"""
    
    def __init__(self, retry_system: UniversalRetrySystem):
        self.retry = retry_system
    
    def cargo_build(self, release: bool = True, clean_first: bool = False) -> bool:
        """Build Rust project with retry"""
        
        def build():
            if clean_first:
                subprocess.run(["cargo", "clean"], check=False)
            
            cmd = ["cargo", "build"]
            if release:
                cmd.append("--release")
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if result.returncode != 0:
                raise Exception(f"Build failed: {result.stderr}")
            
            return result.stdout
        
        success, _, error = self.retry.execute_with_retry("cargo_build", build)
        return success
    
    def cargo_test(self, test_name: str = None) -> bool:
        """Run tests with retry"""
        
        def test():
            cmd = ["cargo", "test", "--lib"]
            if test_name:
                cmd.append(test_name)
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if result.returncode != 0:
                raise Exception(f"Tests failed: {result.stderr}")
            
            return result.stdout
        
        success, _, error = self.retry.execute_with_retry("cargo_test", test)
        return success
    
    def cargo_fmt(self, check: bool = False) -> bool:
        """Format code with retry"""
        
        def fmt():
            cmd = ["cargo", "fmt"]
            if check:
                cmd.extend(["--", "--check"])
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if result.returncode != 0:
                raise Exception(f"Format failed: {result.stderr}")
            
            return result.stdout
        
        success, _, error = self.retry.execute_with_retry("cargo_fmt", fmt)
        return success
    
    def cargo_clippy(self, fix: bool = False) -> bool:
        """Run clippy with retry"""
        
        def clippy():
            cmd = ["cargo", "clippy"]
            if fix:
                cmd.extend(["--fix", "--allow-dirty", "--allow-staged"])
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if result.returncode != 0:
                raise Exception(f"Clippy failed: {result.stderr}")
            
            return result.stdout
        
        success, _, error = self.retry.execute_with_retry("cargo_clippy", clippy)
        return success
    
    def git_operation(self, git_args: List[str]) -> bool:
        """Git operation with retry"""
        cmd = ["git"] + git_args
        success, _, error = self.retry.execute_command_with_retry(
            cmd,
            f"git_{git_args[0]}"
        )
        return success


def main():
    parser = argparse.ArgumentParser(
        description="Universal Retry System with State Persistence"
    )
    
    parser.add_argument("--operation", help="Operation to execute")
    parser.add_argument("--command", nargs="+", help="Command to execute with retry")
    parser.add_argument("--max-attempts", type=int, default=10, help="Maximum retry attempts")
    parser.add_argument("--state-file", default=".retry_state.json", help="State file path")
    parser.add_argument("--summary", action="store_true", help="Show retry state summary")
    parser.add_argument("--cleanup", type=int, help="Cleanup operations older than N days")
    parser.add_argument("--reset", help="Reset specific operation ID")
    
    # Common operations
    parser.add_argument("--build", action="store_true", help="Build with retry")
    parser.add_argument("--test", action="store_true", help="Test with retry")
    parser.add_argument("--fmt", action="store_true", help="Format with retry")
    parser.add_argument("--clippy", action="store_true", help="Clippy with retry")
    
    args = parser.parse_args()
    
    # Initialize retry system
    retry_system = UniversalRetrySystem(
        state_file=args.state_file,
        max_attempts=args.max_attempts
    )
    
    # Handle different modes
    if args.summary:
        summary = retry_system.get_persistent_summary()
        print("\n" + "="*70)
        print("📊 RETRY STATE SUMMARY")
        print("="*70)
        print(f"\nTotal Operations: {summary['total_operations']}")
        print(f"Successful: {summary['successful_operations']}")
        print(f"Failed: {summary['failed_operations']}")
        print(f"Success Rate: {summary['success_rate']*100:.1f}%")
        
        if summary['operations']:
            print("\n📋 Operations Detail:")
            for op_id, op_info in summary['operations'].items():
                print(f"\n  {op_id}:")
                print(f"    Attempts: {op_info['attempts']}")
                print(f"    Successes: {op_info['success_count']}")
                print(f"    Failures: {op_info['failure_count']}")
                print(f"    Last: {op_info.get('last_attempt', 'N/A')}")
        
        return 0
    
    if args.cleanup:
        count = retry_system.cleanup_old_operations(args.cleanup)
        print(f"✅ Cleaned up {count} operations older than {args.cleanup} days")
        return 0
    
    if args.reset:
        retry_system.retry_state.reset_operation(args.reset)
        print(f"✅ Reset operation: {args.reset}")
        return 0
    
    # Common operations
    common_ops = CommonRetryOperations(retry_system)
    
    if args.build:
        success = common_ops.cargo_build()
        return 0 if success else 1
    
    if args.test:
        success = common_ops.cargo_test()
        return 0 if success else 1
    
    if args.fmt:
        success = common_ops.cargo_fmt()
        return 0 if success else 1
    
    if args.clippy:
        success = common_ops.cargo_clippy()
        return 0 if success else 1
    
    if args.command:
        success, stdout, stderr = retry_system.execute_command_with_retry(
            args.command,
            operation_name=args.operation or " ".join(args.command)
        )
        
        if stdout:
            print(stdout)
        if stderr:
            print(stderr, file=sys.stderr)
        
        return 0 if success else 1
    
    # Default: show usage
    parser.print_help()
    return 1


if __name__ == "__main__":
    sys.exit(main())
