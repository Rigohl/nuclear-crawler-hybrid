#!/usr/bin/env python3
"""
🔍 Universal Branch Analyzer & Automator
Analyzes ALL branches individually and applies automation logic to each one
"""

import os
import sys
import subprocess
import json
from pathlib import Path
from typing import Dict, List, Tuple
import argparse
from datetime import datetime, timedelta

class UniversalBranchAnalyzer:
    """Analyzes and automates ALL branches in the repository"""
    
    def __init__(self, repo_path: str = "."):
        self.repo_path = Path(repo_path)
        self.branches_analyzed = []
        self.automation_applied = []
        
    def get_all_branches(self) -> List[str]:
        """Get ALL branches (local and remote)"""
        try:
            # Get remote branches
            result = subprocess.run(
                ["git", "branch", "-r"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            
            branches = []
            for line in result.stdout.split('\n'):
                line = line.strip()
                if line and 'HEAD' not in line:
                    # Remove 'origin/' prefix
                    branch = line.replace('origin/', '')
                    branches.append(branch)
            
            return sorted(set(branches))
        except Exception as e:
            print(f"Error getting branches: {e}")
            return []
    
    def analyze_branch(self, branch: str) -> Dict:
        """Analyze a specific branch comprehensively"""
        print(f"\n{'='*70}")
        print(f"🔍 Analyzing branch: {branch}")
        print(f"{'='*70}")
        
        analysis = {
            "branch": branch,
            "timestamp": datetime.utcnow().isoformat(),
            "health": {},
            "commits": {},
            "files": {},
            "automation": {},
            "issues": []
        }
        
        try:
            # Checkout branch
            subprocess.run(
                ["git", "fetch", "origin", branch],
                capture_output=True,
                cwd=self.repo_path
            )
            
            # Get commit info
            result = subprocess.run(
                ["git", "log", f"origin/{branch}", "-1", "--format=%H|%an|%ae|%at|%s"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            
            if result.returncode == 0 and result.stdout.strip():
                parts = result.stdout.strip().split('|')
                analysis["commits"] = {
                    "last_commit_sha": parts[0] if len(parts) > 0 else "",
                    "author_name": parts[1] if len(parts) > 1 else "",
                    "author_email": parts[2] if len(parts) > 2 else "",
                    "commit_timestamp": int(parts[3]) if len(parts) > 3 else 0,
                    "commit_message": parts[4] if len(parts) > 4 else ""
                }
                
                # Calculate age
                commit_time = datetime.fromtimestamp(analysis["commits"]["commit_timestamp"])
                age_days = (datetime.now() - commit_time).days
                analysis["commits"]["age_days"] = age_days
                
                print(f"   Last commit: {age_days} days ago")
                print(f"   Author: {analysis['commits']['author_name']}")
                print(f"   Message: {analysis['commits']['commit_message'][:50]}...")
            
            # Check if behind main/master
            for main_branch in ['main', 'master']:
                result = subprocess.run(
                    ["git", "rev-list", "--count", f"origin/{branch}..origin/{main_branch}"],
                    capture_output=True,
                    text=True,
                    cwd=self.repo_path
                )
                
                if result.returncode == 0:
                    commits_behind = int(result.stdout.strip() or 0)
                    if commits_behind > 0:
                        analysis["health"]["behind"] = {
                            "base_branch": main_branch,
                            "commits_behind": commits_behind
                        }
                        print(f"   ⚠️ {commits_behind} commits behind {main_branch}")
                        analysis["issues"].append(f"Behind {main_branch} by {commits_behind} commits")
                    break
            
            # Check file count and types
            result = subprocess.run(
                ["git", "ls-tree", "-r", f"origin/{branch}", "--name-only"],
                capture_output=True,
                text=True,
                cwd=self.repo_path
            )
            
            if result.returncode == 0:
                files = result.stdout.strip().split('\n')
                file_types = {}
                
                for file in files:
                    if file:
                        ext = Path(file).suffix or 'no_extension'
                        file_types[ext] = file_types.get(ext, 0) + 1
                
                analysis["files"] = {
                    "total_files": len([f for f in files if f]),
                    "file_types": file_types
                }
                
                print(f"   Files: {analysis['files']['total_files']}")
                
                # Check for important files
                important_files = ['Cargo.toml', 'package.json', 'README.md', '.github/workflows']
                for important in important_files:
                    if any(important in f for f in files):
                        print(f"   ✅ Has {important}")
            
            # Check for conflicts with main
            for main_branch in ['main', 'master']:
                result = subprocess.run(
                    ["git", "merge-tree", 
                     subprocess.run(["git", "merge-base", f"origin/{branch}", f"origin/{main_branch}"],
                                  capture_output=True, text=True, cwd=self.repo_path).stdout.strip(),
                     f"origin/{branch}",
                     f"origin/{main_branch}"],
                    capture_output=True,
                    text=True,
                    cwd=self.repo_path
                )
                
                if "CONFLICT" in result.stdout:
                    analysis["health"]["has_conflicts"] = True
                    analysis["issues"].append(f"Has merge conflicts with {main_branch}")
                    print(f"   ⚠️ Has conflicts with {main_branch}")
                break
            
            # Health status
            health_score = 100
            
            if analysis.get("health", {}).get("behind", {}).get("commits_behind", 0) > 10:
                health_score -= 20
            elif analysis.get("health", {}).get("behind", {}).get("commits_behind", 0) > 0:
                health_score -= 10
            
            if analysis.get("health", {}).get("has_conflicts"):
                health_score -= 30
            
            if analysis["commits"].get("age_days", 0) > 90:
                health_score -= 20
            elif analysis["commits"].get("age_days", 0) > 30:
                health_score -= 10
            
            analysis["health"]["score"] = max(0, health_score)
            
            if health_score >= 80:
                analysis["health"]["status"] = "healthy"
                print(f"   ✅ Health: {health_score}/100 (Healthy)")
            elif health_score >= 50:
                analysis["health"]["status"] = "warning"
                print(f"   ⚠️ Health: {health_score}/100 (Warning)")
            else:
                analysis["health"]["status"] = "critical"
                print(f"   ❌ Health: {health_score}/100 (Critical)")
            
            self.branches_analyzed.append(analysis)
            return analysis
            
        except Exception as e:
            print(f"   ❌ Error analyzing branch: {e}")
            analysis["error"] = str(e)
            return analysis
    
    def apply_automation(self, branch: str, analysis: Dict) -> Dict:
        """Apply automation logic to a specific branch"""
        print(f"\n🤖 Applying automation to: {branch}")
        
        automation_actions = {
            "branch": branch,
            "actions_taken": [],
            "actions_needed": []
        }
        
        # Action 1: Sync if behind
        if analysis.get("health", {}).get("behind", {}).get("commits_behind", 0) > 0:
            commits_behind = analysis["health"]["behind"]["commits_behind"]
            base_branch = analysis["health"]["behind"]["base_branch"]
            
            print(f"   🔄 Action: Sync with {base_branch} ({commits_behind} commits)")
            automation_actions["actions_needed"].append({
                "action": "sync",
                "target": base_branch,
                "commits": commits_behind
            })
        
        # Action 2: Resolve conflicts
        if analysis.get("health", {}).get("has_conflicts"):
            print(f"   ⚔️ Action: Resolve conflicts")
            automation_actions["actions_needed"].append({
                "action": "resolve_conflicts",
                "priority": "high"
            })
        
        # Action 3: Cleanup if stale
        if analysis["commits"].get("age_days", 0) > 90:
            print(f"   🧹 Action: Consider cleanup (stale for {analysis['commits']['age_days']} days)")
            automation_actions["actions_needed"].append({
                "action": "cleanup",
                "reason": "stale",
                "age_days": analysis["commits"]["age_days"]
            })
        
        # Action 4: Health check
        if analysis["health"].get("score", 100) < 70:
            print(f"   🏥 Action: Health check required (score: {analysis['health']['score']})")
            automation_actions["actions_needed"].append({
                "action": "health_check",
                "score": analysis["health"]["score"]
            })
        
        # Action 5: Run validation
        print(f"   ✅ Action: Run validation suite")
        automation_actions["actions_needed"].append({
            "action": "validate",
            "priority": "normal"
        })
        
        self.automation_applied.append(automation_actions)
        return automation_actions
    
    def analyze_all_branches(self) -> Dict:
        """Analyze ALL branches comprehensively"""
        print("\n" + "="*70)
        print("🔍 UNIVERSAL BRANCH ANALYZER - ANALYZING ALL BRANCHES")
        print("="*70)
        
        branches = self.get_all_branches()
        
        if not branches:
            print("❌ No branches found")
            return {"error": "no_branches"}
        
        print(f"\n📋 Found {len(branches)} branches to analyze:")
        for i, branch in enumerate(branches, 1):
            print(f"   {i}. {branch}")
        
        # Analyze each branch
        for branch in branches:
            analysis = self.analyze_branch(branch)
            automation = self.apply_automation(branch, analysis)
        
        # Generate summary
        return self.generate_summary()
    
    def generate_summary(self) -> Dict:
        """Generate comprehensive summary of all branches"""
        print("\n" + "="*70)
        print("📊 SUMMARY - ALL BRANCHES ANALYZED & AUTOMATED")
        print("="*70)
        
        summary = {
            "total_branches": len(self.branches_analyzed),
            "timestamp": datetime.utcnow().isoformat(),
            "health_distribution": {
                "healthy": 0,
                "warning": 0,
                "critical": 0
            },
            "actions_needed": {
                "sync": 0,
                "resolve_conflicts": 0,
                "cleanup": 0,
                "health_check": 0,
                "validate": 0
            },
            "branches_by_age": {
                "0-30_days": 0,
                "30-90_days": 0,
                "90+_days": 0
            }
        }
        
        for branch_data in self.branches_analyzed:
            # Health distribution
            status = branch_data.get("health", {}).get("status", "unknown")
            if status in summary["health_distribution"]:
                summary["health_distribution"][status] += 1
            
            # Age distribution
            age = branch_data.get("commits", {}).get("age_days", 0)
            if age <= 30:
                summary["branches_by_age"]["0-30_days"] += 1
            elif age <= 90:
                summary["branches_by_age"]["30-90_days"] += 1
            else:
                summary["branches_by_age"]["90+_days"] += 1
        
        # Count actions needed
        for automation in self.automation_applied:
            for action in automation.get("actions_needed", []):
                action_type = action.get("action")
                if action_type in summary["actions_needed"]:
                    summary["actions_needed"][action_type] += 1
        
        # Print summary
        print(f"\n📈 Total Branches Analyzed: {summary['total_branches']}")
        
        print(f"\n🏥 Health Distribution:")
        print(f"   ✅ Healthy: {summary['health_distribution']['healthy']}")
        print(f"   ⚠️ Warning: {summary['health_distribution']['warning']}")
        print(f"   ❌ Critical: {summary['health_distribution']['critical']}")
        
        print(f"\n📅 Branch Age Distribution:")
        print(f"   0-30 days: {summary['branches_by_age']['0-30_days']}")
        print(f"   30-90 days: {summary['branches_by_age']['30-90_days']}")
        print(f"   90+ days: {summary['branches_by_age']['90+_days']}")
        
        print(f"\n🤖 Actions Needed:")
        for action, count in summary["actions_needed"].items():
            if count > 0:
                print(f"   {action}: {count} branches")
        
        print(f"\n✅ Analysis Complete!")
        print(f"   - All {summary['total_branches']} branches analyzed")
        print(f"   - Automation logic applied to each branch")
        print(f"   - {sum(summary['actions_needed'].values())} total actions identified")
        
        return summary
    
    def export_report(self, output_file: str):
        """Export comprehensive report to JSON"""
        report = {
            "summary": self.generate_summary(),
            "branches": self.branches_analyzed,
            "automation": self.automation_applied
        }
        
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"\n📄 Report exported to: {output_file}")


def main():
    parser = argparse.ArgumentParser(
        description="Universal Branch Analyzer & Automator - Analyzes ALL branches"
    )
    parser.add_argument("--repo-path", default=".", help="Repository path")
    parser.add_argument("--output", help="Output report file (JSON)")
    parser.add_argument("--branch", help="Analyze specific branch only")
    
    args = parser.parse_args()
    
    analyzer = UniversalBranchAnalyzer(repo_path=args.repo_path)
    
    if args.branch:
        # Analyze specific branch
        print(f"🔍 Analyzing specific branch: {args.branch}")
        analysis = analyzer.analyze_branch(args.branch)
        automation = analyzer.apply_automation(args.branch, analysis)
        
        if args.output:
            with open(args.output, 'w') as f:
                json.dump({
                    "analysis": analysis,
                    "automation": automation
                }, f, indent=2)
    else:
        # Analyze ALL branches
        summary = analyzer.analyze_all_branches()
        
        if args.output:
            analyzer.export_report(args.output)
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
