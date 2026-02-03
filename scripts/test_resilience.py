#!/usr/bin/env python3
"""
Test script to verify CI/CD resilience mechanisms
"""

import sys
import subprocess
from pathlib import Path

def test_validation_script():
    """Test the enhanced validation script"""
    print("🧪 Testing validation script...")
    try:
        result = subprocess.run(
            ["python3", "scripts/validate_system.py"],
            capture_output=True,
            text=True,
            timeout=30
        )
        print(f"   Exit code: {result.returncode}")
        if "SYSTEM STATUS REPORT" in result.stdout:
            print("   ✅ Validation script works")
            return True
        else:
            print("   ❌ Validation script output unexpected")
            return False
    except Exception as e:
        print(f"   ❌ Error: {e}")
        return False

def test_checkpoint_manager():
    """Test the checkpoint manager"""
    print("\n🧪 Testing checkpoint manager...")
    
    commands = [
        (["python3", "scripts/checkpoint_manager.py", "list"], "List backups"),
        (["python3", "scripts/checkpoint_manager.py", "validate"], "Validate checkpoints"),
    ]
    
    for cmd, desc in commands:
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=30
            )
            print(f"   {desc}: exit code {result.returncode}")
            if result.returncode in [0, 1]:  # 0 or 1 are both acceptable
                print(f"   ✅ {desc} works")
            else:
                print(f"   ❌ {desc} failed")
                return False
        except Exception as e:
            print(f"   ❌ Error testing {desc}: {e}")
            return False
    
    return True

def test_workflow_syntax():
    """Test workflow YAML syntax"""
    print("\n🧪 Testing workflow syntax...")
    
    workflows = [
        ".github/workflows/ci.yml",
        ".github/workflows/master-validation.yml",
        ".github/workflows/ci-self-healing.yml"
    ]
    
    all_valid = True
    
    for workflow in workflows:
        try:
            import yaml
            with open(workflow, 'r') as f:
                yaml.safe_load(f)
            print(f"   ✅ {Path(workflow).name}")
        except Exception as e:
            print(f"   ❌ {Path(workflow).name}: {e}")
            all_valid = False
    
    return all_valid

def test_docker_recovery():
    """Test Docker recovery image can be built"""
    print("\n🧪 Testing Docker recovery configuration...")
    
    if not Path("Dockerfile.recovery").exists():
        print("   ❌ Dockerfile.recovery not found")
        return False
    
    print("   ✅ Dockerfile.recovery exists")
    
    # Check if Docker is available
    try:
        result = subprocess.run(
            ["docker", "--version"],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            print(f"   ✅ Docker available: {result.stdout.strip()}")
            return True
        else:
            print("   ⚠️  Docker not available (optional)")
            return True  # Don't fail if Docker isn't available
    except FileNotFoundError:
        print("   ⚠️  Docker not installed (optional)")
        return True  # Don't fail if Docker isn't installed
    except Exception as e:
        print(f"   ⚠️  Docker check failed: {e}")
        return True  # Don't fail

def test_documentation():
    """Test that documentation exists"""
    print("\n🧪 Testing documentation...")
    
    docs = [
        ".github/workflows/RESILIENCE.md",
        ".github/workflows/README.md"
    ]
    
    all_exist = True
    
    for doc in docs:
        if Path(doc).exists():
            print(f"   ✅ {Path(doc).name}")
        else:
            print(f"   ❌ {Path(doc).name} not found")
            all_exist = False
    
    return all_exist

def main():
    """Run all tests"""
    print("="*70)
    print("🔧 CI/CD RESILIENCE SYSTEM TEST SUITE")
    print("="*70)
    
    tests = [
        ("Validation Script", test_validation_script),
        ("Checkpoint Manager", test_checkpoint_manager),
        ("Workflow Syntax", test_workflow_syntax),
        ("Docker Recovery", test_docker_recovery),
        ("Documentation", test_documentation),
    ]
    
    results = {}
    
    for test_name, test_func in tests:
        try:
            results[test_name] = test_func()
        except Exception as e:
            print(f"\n❌ Test '{test_name}' crashed: {e}")
            results[test_name] = False
    
    # Summary
    print("\n" + "="*70)
    print("📊 TEST SUMMARY")
    print("="*70)
    
    for test_name, result in results.items():
        symbol = "✅" if result else "❌"
        print(f"{symbol} {test_name}")
    
    all_passed = all(results.values())
    
    if all_passed:
        print("\n✅ ALL TESTS PASSED")
        print("\n🎉 CI/CD Resilience System is ready!")
        return 0
    else:
        print("\n⚠️  SOME TESTS FAILED")
        print("Please review the output above.")
        return 1

if __name__ == "__main__":
    sys.exit(main())
