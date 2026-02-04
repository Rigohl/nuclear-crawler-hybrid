#!/usr/bin/env python3
"""
Workflow Chain Analyzer - Advanced Agent for CI/CD Orchestration

This agent analyzes workflows in chain, determines dependencies,
execution order, and provides intelligent recommendations.

Features:
- Workflow dependency graph construction
- Execution order optimization
- Failure impact analysis
- Automatic repair suggestions
- Chain continuity validation
"""

import os
import sys
import json
import yaml
import argparse
from pathlib import Path
from typing import Dict, List, Set, Tuple
from collections import defaultdict, deque
from datetime import datetime

class WorkflowChainAnalyzer:
    """Advanced agent for workflow chain analysis and optimization"""
    
    def __init__(self, workflows_dir: str = ".github/workflows"):
        self.workflows_dir = Path(workflows_dir)
        self.workflows = {}
        self.dependencies = defaultdict(list)
        self.reverse_dependencies = defaultdict(list)
        self.triggers = defaultdict(list)
        self.execution_order = []
        
    def discover_workflows(self) -> int:
        """Discover all workflow files"""
        print("🔍 Discovering workflows...")
        
        count = 0
        for workflow_file in self.workflows_dir.glob("*.yml"):
            try:
                with open(workflow_file) as f:
                    workflow_data = yaml.safe_load(f)
                
                if workflow_data:
                    self.workflows[workflow_file.name] = workflow_data
                    count += 1
                    print(f"   ✅ {workflow_file.name}")
            
            except Exception as e:
                print(f"   ⚠️  Error loading {workflow_file.name}: {e}")
        
        print(f"\n✅ Discovered {count} workflows")
        return count
    
    def analyze_dependencies(self):
        """Analyze workflow dependencies"""
        print("\n🔗 Analyzing dependencies...")
        
        for workflow_name, workflow_data in self.workflows.items():
            if 'on' not in workflow_data:
                continue
            
            on_config = workflow_data['on']
            if not isinstance(on_config, dict):
                continue
            
            # Check for workflow_run triggers
            if 'workflow_run' in on_config:
                wf_run = on_config['workflow_run']
                
                if isinstance(wf_run, dict) and 'workflows' in wf_run:
                    workflows_list = wf_run['workflows']
                    if isinstance(workflows_list, list):
                        for dep in workflows_list:
                            self.dependencies[workflow_name].append(dep)
                            self.reverse_dependencies[dep].append(workflow_name)
        
        # Print dependency summary
        if self.dependencies:
            print("\n📊 Dependency Graph:")
            for workflow, deps in self.dependencies.items():
                print(f"   {workflow}")
                for dep in deps:
                    print(f"      ← depends on {dep}")
        else:
            print("   ℹ️  No workflow_run dependencies found")
    
    def analyze_triggers(self):
        """Analyze workflow triggers"""
        print("\n🎯 Analyzing triggers...")
        
        trigger_types = defaultdict(list)
        
        for workflow_name, workflow_data in self.workflows.items():
            if 'on' not in workflow_data:
                continue
            
            on_config = workflow_data['on']
            
            if isinstance(on_config, dict):
                for trigger in on_config.keys():
                    trigger_types[trigger].append(workflow_name)
                    self.triggers[workflow_name].append(trigger)
            elif isinstance(on_config, list):
                for trigger in on_config:
                    trigger_types[trigger].append(workflow_name)
                    self.triggers[workflow_name].append(trigger)
        
        print("\n📊 Trigger Summary:")
        for trigger, workflows in sorted(trigger_types.items(), 
                                         key=lambda x: -len(x[1])):
            print(f"   {trigger}: {len(workflows)} workflows")
            if len(workflows) <= 5:
                for wf in workflows:
                    print(f"      • {wf}")
    
    def calculate_execution_order(self) -> List[List[str]]:
        """Calculate optimal execution order using topological sort"""
        print("\n⛓️ Calculating execution order...")
        
        # Build in-degree map
        in_degree = defaultdict(int)
        for workflow in self.workflows.keys():
            in_degree[workflow] = len(self.dependencies.get(workflow, []))
        
        # Find workflows with no dependencies (level 0)
        queue = deque([wf for wf in self.workflows.keys() if in_degree[wf] == 0])
        
        levels = []
        level_num = 0
        
        while queue:
            level_size = len(queue)
            current_level = []
            
            for _ in range(level_size):
                workflow = queue.popleft()
                current_level.append(workflow)
                
                # Reduce in-degree for dependent workflows
                for dependent in self.reverse_dependencies.get(workflow, []):
                    in_degree[dependent] -= 1
                    if in_degree[dependent] == 0:
                        queue.append(dependent)
            
            if current_level:
                levels.append(current_level)
                level_num += 1
        
        self.execution_order = levels
        
        # Print execution order
        print(f"\n📋 Execution Order ({len(levels)} levels):")
        for i, level in enumerate(levels, 1):
            print(f"\n   Level {i} ({len(level)} workflows):")
            for workflow in level:
                triggers = ', '.join(self.triggers.get(workflow, ['none']))
                print(f"      • {workflow} (triggers: {triggers})")
        
        return levels
    
    def detect_circular_dependencies(self) -> List[List[str]]:
        """Detect circular dependencies in workflow chain"""
        print("\n🔄 Detecting circular dependencies...")
        
        visited = set()
        rec_stack = set()
        cycles = []
        
        def dfs(workflow, path):
            visited.add(workflow)
            rec_stack.add(workflow)
            path.append(workflow)
            
            for dep in self.dependencies.get(workflow, []):
                if dep not in visited:
                    if dfs(dep, path.copy()):
                        return True
                elif dep in rec_stack:
                    # Found cycle
                    cycle_start = path.index(dep)
                    cycle = path[cycle_start:] + [dep]
                    cycles.append(cycle)
                    return True
            
            rec_stack.remove(workflow)
            return False
        
        for workflow in self.workflows.keys():
            if workflow not in visited:
                dfs(workflow, [])
        
        if cycles:
            print(f"   ❌ Found {len(cycles)} circular dependencies:")
            for cycle in cycles:
                print(f"      {' → '.join(cycle)}")
        else:
            print("   ✅ No circular dependencies detected")
        
        return cycles
    
    def analyze_failure_impact(self, failed_workflow: str) -> Dict:
        """Analyze impact if a workflow fails"""
        print(f"\n💥 Analyzing failure impact for: {failed_workflow}")
        
        # Find all workflows that depend on this one (directly or indirectly)
        impacted = set()
        queue = deque([failed_workflow])
        
        while queue:
            current = queue.popleft()
            for dependent in self.reverse_dependencies.get(current, []):
                if dependent not in impacted:
                    impacted.add(dependent)
                    queue.append(dependent)
        
        impact = {
            'failed_workflow': failed_workflow,
            'directly_impacted': self.reverse_dependencies.get(failed_workflow, []),
            'total_impacted': list(impacted),
            'impact_percentage': len(impacted) / len(self.workflows) * 100 if self.workflows else 0
        }
        
        print(f"   Directly impacted: {len(impact['directly_impacted'])} workflows")
        for wf in impact['directly_impacted']:
            print(f"      • {wf}")
        
        print(f"   Total impacted: {len(impact['total_impacted'])} workflows ({impact['impact_percentage']:.1f}%)")
        
        return impact
    
    def validate_chain_continuity(self) -> Dict:
        """Validate that workflow chain has proper continuity"""
        print("\n🔗 Validating chain continuity...")
        
        issues = []
        
        # Check 1: Isolated workflows (no triggers, no dependencies)
        isolated = []
        for workflow_name in self.workflows.keys():
            has_deps = len(self.dependencies.get(workflow_name, [])) > 0
            has_dependents = len(self.reverse_dependencies.get(workflow_name, [])) > 0
            has_auto_triggers = any(t in ['push', 'pull_request', 'schedule'] 
                                   for t in self.triggers.get(workflow_name, []))
            
            if not has_deps and not has_dependents and not has_auto_triggers:
                isolated.append(workflow_name)
        
        if isolated:
            issues.append({
                'type': 'isolated_workflows',
                'count': len(isolated),
                'workflows': isolated,
                'severity': 'warning'
            })
            print(f"   ⚠️  {len(isolated)} isolated workflows (no connections):")
            for wf in isolated:
                print(f"      • {wf}")
        
        # Check 2: Workflows without automatic triggers
        no_auto_trigger = []
        for workflow_name in self.workflows.keys():
            triggers = self.triggers.get(workflow_name, [])
            if not any(t in ['push', 'pull_request', 'schedule', 'workflow_run'] 
                      for t in triggers):
                no_auto_trigger.append(workflow_name)
        
        if no_auto_trigger:
            issues.append({
                'type': 'no_automatic_trigger',
                'count': len(no_auto_trigger),
                'workflows': no_auto_trigger,
                'severity': 'info'
            })
            print(f"   ℹ️  {len(no_auto_trigger)} workflows with only manual triggers:")
            for wf in no_auto_trigger[:5]:
                print(f"      • {wf}")
        
        # Check 3: Verify all dependencies exist
        missing_deps = []
        for workflow_name, deps in self.dependencies.items():
            for dep in deps:
                # Check if dependency file exists
                if not any(wf.startswith(dep) for wf in self.workflows.keys()):
                    missing_deps.append((workflow_name, dep))
        
        if missing_deps:
            issues.append({
                'type': 'missing_dependencies',
                'count': len(missing_deps),
                'dependencies': missing_deps,
                'severity': 'error'
            })
            print(f"   ❌ {len(missing_deps)} missing dependencies:")
            for wf, dep in missing_deps:
                print(f"      • {wf} → {dep} (not found)")
        
        if not issues:
            print("   ✅ All continuity checks passed")
        
        return {
            'valid': len([i for i in issues if i['severity'] == 'error']) == 0,
            'issues': issues
        }
    
    def generate_recommendations(self) -> List[Dict]:
        """Generate intelligent recommendations for workflow chain"""
        print("\n💡 Generating recommendations...")
        
        recommendations = []
        
        # Recommendation 1: Add schedule triggers to critical workflows
        critical_workflows = [wf for wf in self.workflows.keys() 
                            if any(keyword in wf.lower() 
                                  for keyword in ['ci', 'chapel', 'repair', 'heal'])]
        
        for wf in critical_workflows:
            if 'schedule' not in self.triggers.get(wf, []):
                recommendations.append({
                    'type': 'add_schedule_trigger',
                    'workflow': wf,
                    'reason': 'Critical workflow without scheduled execution',
                    'suggestion': 'Add cron schedule for continuous monitoring'
                })
        
        # Recommendation 2: Add failure recovery for isolated workflows
        isolated = [wf for wf in self.workflows.keys()
                   if not self.dependencies.get(wf) and 
                      not self.reverse_dependencies.get(wf)]
        
        if len(isolated) > 3:
            recommendations.append({
                'type': 'connect_isolated',
                'count': len(isolated),
                'reason': 'Many isolated workflows reduce coordination',
                'suggestion': 'Add workflow_run dependencies to create chains'
            })
        
        # Recommendation 3: Balance execution levels
        if self.execution_order:
            max_level_size = max(len(level) for level in self.execution_order)
            if max_level_size > 10:
                recommendations.append({
                    'type': 'parallelize_execution',
                    'level_size': max_level_size,
                    'reason': 'Large execution level may cause bottlenecks',
                    'suggestion': 'Consider splitting into smaller parallel chains'
                })
        
        # Print recommendations
        if recommendations:
            for i, rec in enumerate(recommendations, 1):
                print(f"\n   {i}. {rec['type']}")
                print(f"      Reason: {rec['reason']}")
                print(f"      Suggestion: {rec['suggestion']}")
        else:
            print("   ✅ No recommendations - workflow chain is optimal")
        
        return recommendations
    
    def generate_report(self, output_file: str = None):
        """Generate comprehensive analysis report"""
        report = {
            'timestamp': datetime.now().isoformat(),
            'total_workflows': len(self.workflows),
            'workflow_list': list(self.workflows.keys()),
            'dependencies': dict(self.dependencies),
            'reverse_dependencies': dict(self.reverse_dependencies),
            'triggers': dict(self.triggers),
            'execution_order': self.execution_order,
            'execution_levels': len(self.execution_order),
            'circular_dependencies': self.detect_circular_dependencies(),
            'continuity_validation': self.validate_chain_continuity(),
            'recommendations': self.generate_recommendations()
        }
        
        if output_file:
            with open(output_file, 'w') as f:
                json.dump(report, f, indent=2)
            print(f"\n💾 Report saved to: {output_file}")
        
        return report

def main():
    parser = argparse.ArgumentParser(description='Workflow Chain Analyzer Agent')
    parser.add_argument('--workflows-dir', default='.github/workflows',
                       help='Directory containing workflow files')
    parser.add_argument('--output', '-o', help='Output file for report')
    parser.add_argument('--analyze-failure', help='Analyze impact of specific workflow failure')
    parser.add_argument('--validate-only', action='store_true',
                       help='Only validate continuity, no full analysis')
    
    args = parser.parse_args()
    
    print("🔗 Workflow Chain Analyzer Agent")
    print("=" * 60)
    
    analyzer = WorkflowChainAnalyzer(args.workflows_dir)
    
    # Discover workflows
    count = analyzer.discover_workflows()
    
    if count == 0:
        print("❌ No workflows found")
        return 1
    
    # Analyze
    analyzer.analyze_dependencies()
    analyzer.analyze_triggers()
    
    if not args.validate_only:
        analyzer.calculate_execution_order()
        analyzer.detect_circular_dependencies()
    
    analyzer.validate_chain_continuity()
    
    if args.analyze_failure:
        analyzer.analyze_failure_impact(args.analyze_failure)
    
    if not args.validate_only:
        analyzer.generate_recommendations()
    
    # Generate report
    report = analyzer.generate_report(args.output)
    
    print("\n" + "=" * 60)
    print("✅ Analysis complete")
    print(f"   Workflows: {report['total_workflows']}")
    print(f"   Execution levels: {report['execution_levels']}")
    print(f"   Recommendations: {len(report['recommendations'])}")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())
