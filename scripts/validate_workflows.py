#!/usr/bin/env python3
"""
Comprehensive GitHub Actions Workflow Validator
Validates workflow YAML files for common issues and best practices.
"""
import sys
import yaml
from pathlib import Path
from typing import Dict, List, Tuple
import re


class WorkflowValidator:
    """Validates GitHub Actions workflow files."""
    
    def __init__(self, workflows_dir: Path):
        self.workflows_dir = workflows_dir
        self.errors = []
        self.warnings = []
        self.info = []
        
    def validate_all(self) -> Tuple[int, int, int]:
        """Validate all workflow files. Returns (errors, warnings, info)."""
        for workflow_file in sorted(self.workflows_dir.glob("*.yml")):
            self.validate_workflow(workflow_file)
        
        return len(self.errors), len(self.warnings), len(self.info)
    
    def validate_workflow(self, filepath: Path):
        """Validate a single workflow file."""
        try:
            with open(filepath, 'r') as f:
                content = f.read()
                
            # Check for trailing spaces
            for i, line in enumerate(content.splitlines(), 1):
                if line.endswith(' ') or line.endswith('\t'):
                    self.warnings.append(f"{filepath.name}:{i} - Trailing whitespace")
            
            # Parse YAML
            try:
                workflow = yaml.safe_load(content)
            except yaml.YAMLError as e:
                self.errors.append(f"{filepath.name} - Invalid YAML: {e}")
                return
            
            # Validate structure
            self._validate_structure(filepath.name, workflow)
            self._validate_jobs(filepath.name, workflow)
            self._check_best_practices(filepath.name, workflow, content)
            
        except Exception as e:
            self.errors.append(f"{filepath.name} - Error reading file: {e}")
    
    def _validate_structure(self, filename: str, workflow: Dict):
        """Validate basic workflow structure."""
        if not isinstance(workflow, dict):
            self.errors.append(f"{filename} - Workflow must be a dictionary")
            return
        
        if 'name' not in workflow:
            self.warnings.append(f"{filename} - Missing workflow name")
        
        if 'on' not in workflow:
            self.errors.append(f"{filename} - Missing 'on' trigger configuration")
        
        if 'jobs' not in workflow:
            self.errors.append(f"{filename} - Missing 'jobs' section")
    
    def _validate_jobs(self, filename: str, workflow: Dict):
        """Validate jobs configuration."""
        jobs = workflow.get('jobs', {})
        if not isinstance(jobs, dict):
            self.errors.append(f"{filename} - 'jobs' must be a dictionary")
            return
        
        for job_name, job_config in jobs.items():
            if not isinstance(job_config, dict):
                self.errors.append(f"{filename}:{job_name} - Job config must be a dictionary")
                continue
            
            # Check for runs-on
            if 'runs-on' not in job_config:
                self.errors.append(f"{filename}:{job_name} - Missing 'runs-on'")
            
            # Check for steps
            if 'steps' not in job_config:
                self.warnings.append(f"{filename}:{job_name} - Job has no steps")
            else:
                self._validate_steps(filename, job_name, job_config['steps'])
    
    def _validate_steps(self, filename: str, job_name: str, steps: List):
        """Validate job steps."""
        if not isinstance(steps, list):
            self.errors.append(f"{filename}:{job_name} - 'steps' must be a list")
            return
        
        for i, step in enumerate(steps, 1):
            if not isinstance(step, dict):
                self.errors.append(f"{filename}:{job_name}:step{i} - Step must be a dictionary")
                continue
            
            # Check if step has either 'uses' or 'run'
            if 'uses' not in step and 'run' not in step:
                self.warnings.append(f"{filename}:{job_name}:step{i} - Step missing 'uses' or 'run'")
    
    def _check_best_practices(self, filename: str, workflow: Dict, content: str):
        """Check for best practices and common issues."""
        # Check for deprecated actions
        deprecated_actions = [
            ('actions-rs/toolchain@v1', 'dtolnay/rust-toolchain'),
            ('actions/cache@v2', 'actions/cache@v3 or v4'),
        ]
        
        for old_action, new_action in deprecated_actions:
            if old_action in content:
                self.warnings.append(
                    f"{filename} - Using deprecated action '{old_action}', "
                    f"consider '{new_action}'"
                )
        
        # Check for hardcoded script paths
        if 'python3 scripts/' in content and 'python3 .github/scripts/' not in content:
            self.info.append(
                f"{filename} - Uses 'scripts/' path, ensure fallback to '.github/scripts/'"
            )
        
        # Check for matrix without proper reference
        if '${{ matrix.os }}' in content:
            jobs = workflow.get('jobs', {})
            has_matrix = False
            for job_config in jobs.values():
                if isinstance(job_config, dict):
                    strategy = job_config.get('strategy', {})
                    if isinstance(strategy, dict) and 'matrix' in strategy:
                        has_matrix = True
                        break
            
            if not has_matrix:
                self.errors.append(
                    f"{filename} - References '${{{{ matrix.os }}}}' without matrix strategy"
                )
        
        # Check for missing continue-on-error on known problematic commands
        problematic_patterns = [
            'cargo build',
            'cargo test',
            'cargo clippy',
        ]
        
        for pattern in problematic_patterns:
            if pattern in content and 'continue-on-error' not in content:
                self.info.append(
                    f"{filename} - Contains '{pattern}' without 'continue-on-error', "
                    "consider adding error handling"
                )
        
        # Check for missing error handling in build steps
        if 'cargo build' in content and 'bincode' not in content:
            self.info.append(
                f"{filename} - Contains cargo build, consider filtering bincode errors"
            )
    
    def print_results(self):
        """Print validation results."""
        if self.errors:
            print("\n❌ ERRORS:")
            for error in self.errors:
                print(f"  {error}")
        
        if self.warnings:
            print("\n⚠️  WARNINGS:")
            for warning in self.warnings:
                print(f"  {warning}")
        
        if self.info:
            print("\nℹ️  INFO:")
            for info in self.info:
                print(f"  {info}")
        
        # Summary
        print(f"\n{'='*60}")
        print(f"Summary: {len(self.errors)} errors, {len(self.warnings)} warnings, {len(self.info)} info")
        
        if self.errors:
            print("❌ Validation FAILED")
            return 1
        elif self.warnings:
            print("⚠️  Validation PASSED with warnings")
            return 0
        else:
            print("✅ Validation PASSED")
            return 0


def main():
    """Main entry point."""
    workflows_dir = Path('.github/workflows')
    
    if not workflows_dir.exists():
        print(f"❌ Error: {workflows_dir} not found")
        return 1
    
    print("🔍 Validating GitHub Actions workflows...")
    print(f"Location: {workflows_dir.absolute()}")
    print("="*60)
    
    validator = WorkflowValidator(workflows_dir)
    errors, warnings, info = validator.validate_all()
    
    print(f"\n📊 Analyzed {len(list(workflows_dir.glob('*.yml')))} workflow files")
    validator.print_results()
    
    return validator.print_results()


if __name__ == '__main__':
    sys.exit(main())
