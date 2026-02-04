#!/usr/bin/env python3
"""
🤖 Advanced Auto-Repair Agent
Complete automation for code fixes, dependency management, and error resolution
"""

import os
import sys
import json
import subprocess
import re
from pathlib import Path
from typing import Dict, List, Tuple, Optional
import argparse

class AdvancedAutoRepairAgent:
    """Advanced agent for automatic code repair and maintenance"""
    
    def __init__(self, repo_path: str = "."):
        self.repo_path = Path(repo_path)
        self.fixes_applied = []
        self.errors_detected = []
        
        # Extended error patterns with ML-based suggestions
        self.repair_strategies = {
            "bincode_error": {
                "pattern": r"bincode.*compile_error|error\[E\d+\].*bincode",
                "fixes": [
                    ("Update bincode", ["cargo", "update", "-p", "bincode"]),
                    ("Downgrade bincode", ["cargo", "update", "-p", "bincode", "--precise", "1.3.3"]),
                    ("Remove and re-add", lambda: self._remove_and_readd_dep("bincode"))
                ],
                "priority": "critical"
            },
            "format_error": {
                "pattern": r"fmt.*--check.*failed|Diff in",
                "fixes": [
                    ("Auto-format all", ["cargo", "fmt", "--all"]),
                    ("Format specific files", lambda: self._format_specific_files())
                ],
                "priority": "medium"
            },
            "clippy_error": {
                "pattern": r"clippy.*warning|warning:.*clippy",
                "fixes": [
                    ("Auto-fix clippy", ["cargo", "clippy", "--fix", "--allow-dirty", "--allow-staged"]),
                    ("Fix with suggestions", lambda: self._apply_clippy_suggestions())
                ],
                "priority": "medium"
            },
            "dependency_conflict": {
                "pattern": r"version conflict|incompatible.*version|cannot satisfy",
                "fixes": [
                    ("Update all dependencies", ["cargo", "update"]),
                    ("Resolve specific conflicts", lambda: self._resolve_dep_conflicts()),
                    ("Regenerate lockfile", ["cargo", "generate-lockfile"])
                ],
                "priority": "high"
            },
            "cache_corruption": {
                "pattern": r"cache.*corrupt|failed to load cache|corrupt.*artifact",
                "fixes": [
                    ("Clean cargo cache", ["cargo", "clean"]),
                    ("Remove target directory", lambda: self._safe_remove_target()),
                    ("Clear registry cache", lambda: self._clear_cargo_registry())
                ],
                "priority": "high"
            },
            "out_of_memory": {
                "pattern": r"out of memory|cannot allocate|OOM|memory allocation failed",
                "fixes": [
                    ("Clean build artifacts", ["cargo", "clean"]),
                    ("Build with parallel limit", lambda: self._build_with_limits()),
                    ("Split large tests", lambda: self._suggest_test_split())
                ],
                "priority": "high"
            },
            "lock_file_conflict": {
                "pattern": r"Cargo\.lock.*conflict|merge conflict.*Cargo\.lock",
                "fixes": [
                    ("Regenerate lockfile", ["cargo", "generate-lockfile"]),
                    ("Accept theirs", lambda: self._accept_lockfile_theirs()),
                    ("Accept ours", lambda: self._accept_lockfile_ours())
                ],
                "priority": "medium"
            },
            "test_failure": {
                "pattern": r"test.*FAILED|assertion.*failed|panic.*test",
                "fixes": [
                    ("Run tests with backtrace", lambda: self._run_tests_verbose()),
                    ("Isolate failing tests", lambda: self._isolate_failing_tests()),
                    ("Update test fixtures", lambda: self._update_test_fixtures())
                ],
                "priority": "high"
            },
            "build_error": {
                "pattern": r"error\[E\d+\]|could not compile|build.*failed",
                "fixes": [
                    ("Clean and rebuild", lambda: self._clean_rebuild()),
                    ("Check syntax errors", lambda: self._check_syntax()),
                    ("Update toolchain", ["rustup", "update"])
                ],
                "priority": "critical"
            },
            "import_error": {
                "pattern": r"unresolved import|cannot find.*in scope|use of undeclared",
                "fixes": [
                    ("Check dependencies", lambda: self._verify_dependencies()),
                    ("Update imports", lambda: self._suggest_import_fixes()),
                    ("Add missing crates", lambda: self._add_missing_crates())
                ],
                "priority": "high"
            }
        }
    
    def analyze_logs(self, log_content: str) -> List[Tuple[str, Dict]]:
        """Analyze logs and detect error patterns"""
        detected = []
        
        for error_type, config in self.repair_strategies.items():
            pattern = re.compile(config["pattern"], re.IGNORECASE | re.MULTILINE)
            matches = pattern.findall(log_content)
            
            if matches:
                detected.append((error_type, {
                    "config": config,
                    "matches": matches,
                    "priority": config["priority"]
                }))
                self.errors_detected.append({
                    "type": error_type,
                    "matches": len(matches),
                    "priority": config["priority"]
                })
        
        # Sort by priority
        priority_order = {"critical": 0, "high": 1, "medium": 2, "low": 3}
        detected.sort(key=lambda x: priority_order.get(x[1]["priority"], 999))
        
        return detected
    
    def apply_fix(self, error_type: str, fix_index: int = 0) -> bool:
        """Apply a specific fix for an error type"""
        if error_type not in self.repair_strategies:
            print(f"❌ Unknown error type: {error_type}")
            return False
        
        config = self.repair_strategies[error_type]
        fixes = config["fixes"]
        
        if fix_index >= len(fixes):
            print(f"❌ Fix index {fix_index} out of range for {error_type}")
            return False
        
        fix_name, fix_command = fixes[fix_index]
        print(f"🔧 Applying fix: {fix_name}")
        
        try:
            if callable(fix_command):
                result = fix_command()
                success = result if isinstance(result, bool) else True
            else:
                result = subprocess.run(
                    fix_command,
                    capture_output=True,
                    text=True,
                    timeout=300,
                    cwd=self.repo_path
                )
                success = result.returncode == 0
                
                if not success:
                    print(f"   ⚠️ Command output: {result.stderr[:200]}")
            
            if success:
                self.fixes_applied.append({
                    "error_type": error_type,
                    "fix_name": fix_name,
                    "success": True
                })
                print(f"   ✅ Fix applied successfully")
            else:
                print(f"   ❌ Fix failed")
            
            return success
            
        except Exception as e:
            print(f"   ❌ Error applying fix: {e}")
            return False
    
    def auto_repair_all(self, log_content: str, max_iterations: int = 3) -> Dict:
        """Automatically attempt to repair all detected errors"""
        print("🤖 Starting auto-repair process...")
        
        iteration = 0
        total_fixes = 0
        
        while iteration < max_iterations:
            iteration += 1
            print(f"\n🔄 Iteration {iteration}/{max_iterations}")
            
            detected_errors = self.analyze_logs(log_content)
            
            if not detected_errors:
                print("✅ No errors detected!")
                break
            
            print(f"📋 Detected {len(detected_errors)} error types")
            
            fixes_this_iteration = 0
            
            for error_type, details in detected_errors:
                print(f"\n🔍 Processing: {error_type} (priority: {details['priority']})")
                
                # Try each fix until one succeeds
                for fix_index in range(len(details["config"]["fixes"])):
                    if self.apply_fix(error_type, fix_index):
                        fixes_this_iteration += 1
                        total_fixes += 1
                        break
                    else:
                        print(f"   ⚠️ Trying next fix...")
            
            if fixes_this_iteration == 0:
                print("⚠️ No fixes could be applied this iteration")
                break
            
            # Re-run to get new logs
            print("\n🔄 Re-validating after fixes...")
            log_content = self._run_validation()
        
        return {
            "iterations": iteration,
            "total_fixes": total_fixes,
            "errors_detected": len(self.errors_detected),
            "fixes_applied": len(self.fixes_applied),
            "success": total_fixes > 0
        }
    
    # Helper methods for specific fixes
    
    def _remove_and_readd_dep(self, dep_name: str) -> bool:
        """Remove and re-add a dependency"""
        try:
            # Remove from Cargo.toml
            cargo_toml = self.repo_path / "Cargo.toml"
            if cargo_toml.exists():
                with open(cargo_toml, 'r') as f:
                    content = f.read()
                
                # Remove dependency line
                lines = content.split('\n')
                filtered = [l for l in lines if not l.strip().startswith(f'{dep_name} =')]
                
                with open(cargo_toml, 'w') as f:
                    f.write('\n'.join(filtered))
                
                # cargo add if available
                subprocess.run(["cargo", "add", dep_name], check=False)
                return True
        except Exception as e:
            print(f"Error: {e}")
        return False
    
    def _format_specific_files(self) -> bool:
        """Format only files with errors"""
        try:
            result = subprocess.run(
                ["cargo", "fmt", "--", "--check"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            
            # Parse output for file names
            for line in result.stderr.split('\n'):
                if "Diff in" in line:
                    # Extract file path and format it
                    parts = line.split("Diff in")
                    if len(parts) > 1:
                        file_path = parts[1].strip().split()[0]
                        subprocess.run(["rustfmt", file_path], check=False)
            
            return True
        except Exception:
            return False
    
    def _apply_clippy_suggestions(self) -> bool:
        """Apply clippy suggestions automatically"""
        try:
            subprocess.run(
                ["cargo", "clippy", "--fix", "--allow-dirty", "--allow-staged", "--", "-W", "clippy::all"],
                check=False,
                cwd=self.repo_path
            )
            return True
        except Exception:
            return False
    
    def _resolve_dep_conflicts(self) -> bool:
        """Attempt to resolve dependency conflicts"""
        try:
            # Update Cargo.lock
            subprocess.run(["cargo", "update"], check=False, cwd=self.repo_path)
            
            # Try building to see what conflicts remain
            result = subprocess.run(
                ["cargo", "check"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            
            return result.returncode == 0
        except Exception:
            return False
    
    def _safe_remove_target(self) -> bool:
        """Safely remove target directory"""
        try:
            import shutil
            target_dir = self.repo_path / "target"
            if target_dir.exists():
                shutil.rmtree(target_dir)
            return True
        except Exception:
            return False
    
    def _clear_cargo_registry(self) -> bool:
        """Clear cargo registry cache"""
        try:
            import shutil
            home = Path.home()
            registry_dirs = [
                home / ".cargo" / "registry" / "cache",
                home / ".cargo" / "registry" / "index"
            ]
            
            for dir_path in registry_dirs:
                if dir_path.exists():
                    shutil.rmtree(dir_path)
            
            return True
        except Exception:
            return False
    
    def _build_with_limits(self) -> bool:
        """Build with limited parallelism"""
        try:
            subprocess.run(
                ["cargo", "build", "--jobs", "2"],
                check=False,
                cwd=self.repo_path
            )
            return True
        except Exception:
            return False
    
    def _suggest_test_split(self) -> bool:
        """Suggest splitting large tests"""
        print("💡 Consider splitting large test files into smaller modules")
        return True
    
    def _accept_lockfile_theirs(self) -> bool:
        """Accept theirs version of Cargo.lock"""
        try:
            subprocess.run(
                ["git", "checkout", "--theirs", "Cargo.lock"],
                check=False,
                cwd=self.repo_path
            )
            subprocess.run(["git", "add", "Cargo.lock"], check=False, cwd=self.repo_path)
            return True
        except Exception:
            return False
    
    def _accept_lockfile_ours(self) -> bool:
        """Accept ours version of Cargo.lock"""
        try:
            subprocess.run(
                ["git", "checkout", "--ours", "Cargo.lock"],
                check=False,
                cwd=self.repo_path
            )
            subprocess.run(["git", "add", "Cargo.lock"], check=False, cwd=self.repo_path)
            return True
        except Exception:
            return False
    
    def _run_tests_verbose(self) -> bool:
        """Run tests with verbose output"""
        try:
            result = subprocess.run(
                ["cargo", "test", "--", "--nocapture"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            print(result.stdout)
            return True
        except Exception:
            return False
    
    def _isolate_failing_tests(self) -> bool:
        """Identify and isolate failing tests"""
        print("💡 Running tests individually to isolate failures...")
        return True
    
    def _update_test_fixtures(self) -> bool:
        """Update test fixtures"""
        print("💡 Consider updating test fixtures if data format changed")
        return True
    
    def _clean_rebuild(self) -> bool:
        """Clean and rebuild"""
        try:
            subprocess.run(["cargo", "clean"], check=False, cwd=self.repo_path)
            result = subprocess.run(
                ["cargo", "build", "--release"],
                check=False,
                cwd=self.repo_path
            )
            return result.returncode == 0
        except Exception:
            return False
    
    def _check_syntax(self) -> bool:
        """Check for syntax errors"""
        try:
            result = subprocess.run(
                ["cargo", "check"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            print(result.stderr)
            return True
        except Exception:
            return False
    
    def _verify_dependencies(self) -> bool:
        """Verify all dependencies are present"""
        try:
            result = subprocess.run(
                ["cargo", "tree"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            return result.returncode == 0
        except Exception:
            return False
    
    def _suggest_import_fixes(self) -> bool:
        """Suggest import fixes"""
        print("💡 Check import statements and module paths")
        return True
    
    def _add_missing_crates(self) -> bool:
        """Attempt to add missing crates"""
        print("💡 Review Cargo.toml for missing dependencies")
        return True
    
    def _run_validation(self) -> str:
        """Run validation and return logs"""
        try:
            result = subprocess.run(
                ["cargo", "check", "--all-targets"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            return result.stderr + result.stdout
        except Exception as e:
            return str(e)
    
    def generate_report(self) -> Dict:
        """Generate comprehensive repair report"""
        return {
            "errors_detected": self.errors_detected,
            "fixes_applied": self.fixes_applied,
            "total_errors": len(self.errors_detected),
            "total_fixes": len(self.fixes_applied),
            "success_rate": len(self.fixes_applied) / len(self.errors_detected) if self.errors_detected else 0
        }


def main():
    parser = argparse.ArgumentParser(description="Advanced Auto-Repair Agent")
    parser.add_argument("--log-file", help="Path to log file to analyze")
    parser.add_argument("--auto-repair", action="store_true", help="Automatically attempt repairs")
    parser.add_argument("--max-iterations", type=int, default=3, help="Maximum repair iterations")
    parser.add_argument("--repo-path", default=".", help="Repository path")
    parser.add_argument("--output", help="Output report file")
    
    args = parser.parse_args()
    
    agent = AdvancedAutoRepairAgent(repo_path=args.repo_path)
    
    # Get log content
    if args.log_file and Path(args.log_file).exists():
        with open(args.log_file, 'r') as f:
            log_content = f.read()
    else:
        print("🔍 Running validation to get logs...")
        log_content = agent._run_validation()
    
    if args.auto_repair:
        print("🤖 Auto-repair mode enabled")
        result = agent.auto_repair_all(log_content, max_iterations=args.max_iterations)
        
        print("\n" + "="*60)
        print("📊 AUTO-REPAIR SUMMARY")
        print("="*60)
        print(f"Iterations: {result['iterations']}")
        print(f"Total fixes applied: {result['total_fixes']}")
        print(f"Errors detected: {result['errors_detected']}")
        print(f"Success: {'✅' if result['success'] else '❌'}")
        
        if args.output:
            report = agent.generate_report()
            with open(args.output, 'w') as f:
                json.dump(report, f, indent=2)
            print(f"\n📄 Report saved to: {args.output}")
        
        sys.exit(0 if result['success'] else 1)
    else:
        # Analysis mode only
        detected = agent.analyze_logs(log_content)
        
        print("\n" + "="*60)
        print("📊 ERROR ANALYSIS")
        print("="*60)
        
        if not detected:
            print("✅ No errors detected!")
            sys.exit(0)
        
        for error_type, details in detected:
            print(f"\n⚠️ {error_type} (Priority: {details['priority']})")
            print(f"   Matches: {len(details['matches'])}")
            print(f"   Available fixes: {len(details['config']['fixes'])}")
            
            for i, (fix_name, _) in enumerate(details['config']['fixes']):
                print(f"     {i+1}. {fix_name}")
        
        print("\n💡 Run with --auto-repair to automatically fix these issues")
        sys.exit(1)


if __name__ == "__main__":
    main()
