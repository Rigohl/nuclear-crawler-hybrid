# 🔧 CI/CD Resilience & Self-Healing System

This document describes the comprehensive resilience and self-healing mechanisms implemented in the Nuclear Crawler Hybrid CI/CD pipeline.

## 🎯 Overview

The CI/CD pipeline now includes multiple layers of automatic error detection, repair, and recovery mechanisms to minimize manual intervention and maximize pipeline uptime.

## 📋 Table of Contents

1. [Automatic Retry Mechanisms](#automatic-retry-mechanisms)
2. [Error Detection & Auto-Repair](#error-detection--auto-repair)
3. [Intelligent Agent Monitoring](#intelligent-agent-monitoring)
4. [Model Checkpoint Management](#model-checkpoint-management)
5. [Failure Notifications](#failure-notifications)
6. [Usage Guide](#usage-guide)

---

## 🔄 Automatic Retry Mechanisms

### Main CI Workflow (`ci.yml`)

The main CI workflow now includes automatic retry logic for common failure scenarios:

#### Format Check with Auto-Fix
```yaml
- name: 📝 Check Format
  id: fmt-check
  run: cargo fmt -- --check
  continue-on-error: true

- name: 🔧 Auto-Fix Format (on failure)
  if: steps.fmt-check.outcome == 'failure'
  run: |
    echo "⚠️ Formatting issues detected, auto-fixing..."
    cargo fmt
    cargo fmt -- --check
```

#### Build with Cache Rebuild
```yaml
- name: 🔨 Build
  id: build
  run: cargo build --release --verbose
  continue-on-error: true

- name: 🔧 Retry Build with Clean (on failure)
  if: steps.build.outcome == 'failure'
  run: |
    echo "⚠️ Build failed, attempting recovery..."
    cargo clean
    cargo update || true
    cargo build --release --verbose
```

### Master Validation Workflow (`master-validation.yml`)

Similar retry mechanisms with enhanced cache management:

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    key: build-v2
    restore-keys: |
      ${{ runner.os }}-cargo-build-
```

---

## 🔍 Error Detection & Auto-Repair

### Enhanced Validation Script

The `scripts/validate_system.py` has been enhanced with comprehensive error pattern detection and automatic repair capabilities.

#### Supported Error Patterns

1. **Bincode Compile Error**
   - Pattern: `bincode.*compile_error`
   - Repair: `cargo update -p bincode`

2. **Formatting Issues**
   - Pattern: `fmt.*--check.*failed`
   - Repair: `cargo fmt`

3. **Clippy Warnings**
   - Pattern: `clippy.*warning`
   - Repair: `cargo clippy --fix --allow-dirty`

4. **Cache Corruption**
   - Pattern: `cache.*corrupt`
   - Repair: `cargo clean`

5. **Dependency Resolution**
   - Pattern: `failed to resolve`
   - Repair: `cargo update`

6. **Out of Memory**
   - Pattern: `out of memory`
   - Repair: `cargo clean`

7. **Lock File Conflicts**
   - Pattern: `Cargo.lock.*conflict`
   - Repair: `cargo generate-lockfile`

#### Usage

```bash
# Run basic validation
python3 scripts/validate_system.py

# Run enhanced validation with auto-repair
python3 scripts/validate_system.py --enhanced

# Run with auto-repair enabled
python3 scripts/validate_system.py --auto-repair
```

#### Auto-Repair Flow

```
┌─────────────────────┐
│  Error Detection    │
│  (Log Analysis)     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Pattern Matching   │
│  (7 Error Types)    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Apply Repair       │
│  (Automated Fix)    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Validation         │
│  (Verify Success)   │
└─────────────────────┘
```

---

## 🤖 Intelligent Agent Monitoring

### CI Self-Healing Workflow (`ci-self-healing.yml`)

A dedicated workflow monitors main pipelines and automatically attempts repairs when failures are detected.

#### Triggers

1. **Workflow Completion**: Activates when main workflows fail
2. **Scheduled**: Runs every 30 minutes for proactive monitoring
3. **Manual**: Can be triggered manually via workflow_dispatch

#### Monitoring Jobs

##### 1. Monitor & Heal
- Analyzes failed workflow runs
- Attempts cache rebuild
- Updates dependencies
- Applies formatting fixes
- Runs enhanced validation
- Attempts recovery build
- Commits successful repairs (with [skip ci])

##### 2. Backup Checkpoints
- Scans for Chapel AI model checkpoints
- Creates compressed backups
- Uploads as artifacts (90-day retention)
- Maintains backup metadata

##### 3. Docker Health Check
- Tests Docker build health
- Clears Docker cache on failure
- Retries build after cache clear

#### Self-Healing Process Flow

```
GitHub Workflow Failure
         ↓
CI Self-Healing Triggered
         ↓
┌────────────────────────┐
│ 1. Analyze Failure     │
│    - Download logs     │
│    - Detect patterns   │
└───────────┬────────────┘
            ↓
┌────────────────────────┐
│ 2. Attempt Repairs     │
│    - Cache rebuild     │
│    - Dependency update │
│    - Format fix        │
└───────────┬────────────┘
            ↓
┌────────────────────────┐
│ 3. Recovery Build      │
│    - Test with cargo   │
│    - Validate results  │
└───────────┬────────────┘
            ↓
    ┌───────┴────────┐
    │                │
  Success         Failure
    │                │
    ↓                ↓
Commit Fix    Create Issue
```

---

## 💾 Model Checkpoint Management

### Checkpoint Manager (`scripts/checkpoint_manager.py`)

A comprehensive system for managing ML model checkpoints with backup, validation, and recovery capabilities.

#### Features

1. **Automatic Discovery**: Finds all checkpoint files (`.checkpoint`, `.model`, `.weights`, etc.)
2. **Checksum Validation**: Detects corrupted files
3. **Incremental Backups**: Only backup changed files
4. **Compressed Storage**: Creates `.tar.gz` archives
5. **Metadata Tracking**: JSON metadata with checksums and timestamps
6. **Selective Restoration**: Restore specific checkpoints or entire backups

#### Commands

```bash
# Create full backup
python3 scripts/checkpoint_manager.py backup

# Create incremental backup (only changed files)
python3 scripts/checkpoint_manager.py backup --incremental

# List all backups
python3 scripts/checkpoint_manager.py list

# Validate all checkpoints
python3 scripts/checkpoint_manager.py validate

# Restore latest backup
python3 scripts/checkpoint_manager.py restore

# Restore specific backup
python3 scripts/checkpoint_manager.py restore --timestamp 20240203_123456

# Restore specific checkpoint file
python3 scripts/checkpoint_manager.py restore --checkpoint model.weights

# Cleanup old backups (keep last 5)
python3 scripts/checkpoint_manager.py cleanup --keep 5
```

#### Docker Recovery Image

A specialized Docker image for checkpoint recovery:

```bash
# Build recovery image
docker build -f Dockerfile.recovery -t nuclear-recovery .

# Validate checkpoints
docker run -v $(pwd):/recovery nuclear-recovery validate

# Create backup
docker run -v $(pwd):/recovery nuclear-recovery backup

# Restore from backup
docker run -v $(pwd):/recovery nuclear-recovery restore --timestamp 20240203_123456
```

#### Checkpoint Backup in CI

The `ci-self-healing.yml` workflow automatically:
- Runs every 30 minutes
- Scans for new/changed checkpoints
- Creates backups as artifacts
- Retains backups for 90 days

---

## 🚨 Failure Notifications

### Intelligent Issue Creation

When automatic repairs fail, the system creates detailed GitHub Issues with:

1. **Failure Classification**: Categorizes the type of failure
2. **Repair History**: Lists all attempted repairs
3. **Recommended Actions**: Provides specific next steps
4. **Log References**: Links to failed workflow runs
5. **Context**: Includes timestamps and environment details

#### Supported Failure Types

| Failure Type | Detection Pattern | Recommended Actions |
|-------------|------------------|---------------------|
| `bincode_error` | `bincode` in logs | Pin bincode version, use alternatives |
| `format_error` | `fmt.*--check` | Run `cargo fmt` locally |
| `memory_error` | `out of memory` | Split tests, increase resources |
| `unknown` | Other patterns | Review logs, test locally |

#### Issue Creation Logic

```javascript
// Only creates NEW issue if one doesn't exist
// Otherwise, adds comment to existing issue
if (existingIssue) {
  await github.rest.issues.createComment({
    issue_number: existingIssue.number,
    body: "🔄 Failure Recurred\n\n" + issue_body
  });
} else {
  await github.rest.issues.create({
    title: "🚨 CI/CD Failure: ${detected_issue}",
    body: issue_body,
    labels: ['ci-failure', 'auto-created', 'needs-investigation']
  });
}
```

#### Issue Template

```markdown
## 🚨 CI/CD Failure - Manual Intervention Required

**Failed Workflow:** Main CI Pipeline
**Run ID:** 1234567890
**Detected Issue:** bincode_error
**Timestamp:** 2024-02-03T12:34:56Z

### 🔍 Problem Analysis

The self-healing system attempted automatic repairs but was unable to resolve the issue.

**Detected Issue Type:** `bincode_error`

### 🔧 Auto-Repair Attempts

- ✅ Cache rebuild attempted
- ✅ Dependency update attempted
- ✅ Code formatting attempted
- ❌ Recovery build failed

### 📝 Recommended Actions

1. Review the bincode dependency issue in `Cargo.toml`
2. Consider pinning to a specific version: `bincode = "1.3.3"`
3. Check for alternative serialization libraries
4. Review repository memories for known workarounds

### 📊 Build Logs

See the [failed workflow run](link) for detailed logs.
```

---

## 📖 Usage Guide

### For Developers

#### Local Development

1. **Run validation before committing:**
   ```bash
   python3 scripts/validate_system.py --enhanced
   ```

2. **Create checkpoint backup:**
   ```bash
   python3 scripts/checkpoint_manager.py backup
   ```

3. **Validate your changes:**
   ```bash
   cargo fmt
   cargo clippy --fix --allow-dirty
   cargo test
   ```

#### Responding to CI Failures

1. **Check Self-Healing Results**: Review the CI Self-Healing workflow run
2. **Review Auto-Created Issues**: Check for issues labeled `ci-failure`
3. **Apply Recommended Fixes**: Follow the guidance in the issue
4. **Test Locally**: Reproduce and fix the issue locally
5. **Close Issue**: Close the issue when fixed

### For CI/CD Maintainers

#### Monitoring

1. **Review Self-Healing Metrics**:
   - Check workflow run history for `ci-self-healing.yml`
   - Review recovery success rate
   - Monitor issue creation frequency

2. **Checkpoint Management**:
   - Verify backup artifacts are being created
   - Ensure backups are retained appropriately
   - Test restoration process periodically

3. **Error Pattern Updates**:
   - Add new error patterns to `validate_system.py` as discovered
   - Update repair commands based on effectiveness
   - Document common issues in repository memories

#### Configuration

##### Retry Behavior
- Modify `continue-on-error` and retry steps in workflows
- Adjust timeout values for long-running operations
- Configure cache versioning (`v2`, `v3`, etc.)

##### Notification Thresholds
- Update issue creation logic in `ci-self-healing.yml`
- Configure labels and assignees for auto-created issues
- Adjust schedule frequency for monitoring

##### Backup Retention
- Modify `retention-days` in artifact upload steps
- Configure cleanup policy in checkpoint manager
- Adjust backup frequency in schedule

---

## 🔧 Troubleshooting

### Common Issues

#### 1. Self-Healing Workflow Not Triggering

**Symptom**: Self-healing workflow doesn't run after CI failure

**Solutions**:
- Verify `workflow_run` trigger is configured correctly
- Check workflow permissions (needs `issues: write`)
- Ensure main workflow names match exactly

#### 2. Checkpoint Backups Failing

**Symptom**: No checkpoint files found or backup fails

**Solutions**:
- Verify Chapel AI is generating checkpoints
- Check file permissions
- Ensure backup directory is writable
- Review checkpoint patterns in `checkpoint_manager.py`

#### 3. Auto-Repairs Not Applying

**Symptom**: Repairs detected but not applied

**Solutions**:
- Check if `[skip ci]` is preventing commits
- Verify git configuration in workflow
- Ensure proper write permissions
- Review error patterns in validation script

#### 4. Issues Not Being Created

**Symptom**: Failures occur but no issues are created

**Solutions**:
- Verify `issues: write` permission
- Check GitHub token availability
- Review issue creation logic in workflow
- Ensure labels exist in repository

---

## 📊 Metrics & Reporting

### Available Metrics

1. **Recovery Success Rate**: Percentage of failures auto-repaired
2. **Checkpoint Backup Status**: Number of successful backups
3. **Issue Creation Rate**: Auto-created issues per week
4. **Retry Effectiveness**: Success rate of retry attempts

### Health Report

The system generates a comprehensive health report on each run:

```markdown
## 🏥 CI/CD Health Report

**Workflow:** Main CI Pipeline
**Status:** failure
**Timestamp:** 2024-02-03 12:34:56 UTC

### 🔧 Auto-Repair Actions Taken

- ✅ Dependencies updated
- ✅ Code formatting applied
- ✅ Recovery build successful

### 📋 Validation Results

- ✅ System validation passed
```

---

## 🚀 Future Enhancements

### Planned Features

1. **ML-Based Failure Prediction**: Use Chapel AI to predict likely failures
2. **Slack/Discord Integration**: Real-time notifications
3. **Performance Regression Detection**: Automatic benchmarking
4. **Dependency Vulnerability Scanning**: Proactive security updates
5. **Multi-Cloud Backup**: Sync checkpoints to S3/Azure/GCS

### Contributing

To add new error patterns or repair strategies:

1. Add pattern to `CIAutoRepair` class in `validate_system.py`
2. Define repair command
3. Test locally
4. Document in this README
5. Submit PR with tests

---

## 📚 Related Documentation

- [Quick Start Guide](../QUICK_START.md)
- [CI/CD Workflows](../README.md)
- [Chapel AI Integration](../CHAPEL_AI_INTEGRATION_STATUS.md)
- [Copilot Instructions](copilot-instructions.md)

---

## 🆘 Support

If you encounter issues with the resilience system:

1. Check this documentation
2. Review auto-created issues
3. Check workflow run logs
4. Contact the maintainers

---

**Last Updated**: 2024-02-03  
**Version**: 1.0  
**Status**: ✅ Production Ready
