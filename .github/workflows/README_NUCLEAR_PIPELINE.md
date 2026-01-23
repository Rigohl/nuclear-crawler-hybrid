# 🚀 Nuclear Multi-Agent CI/CD Pipeline

## Overview

The Nuclear Multi-Agent CI/CD Pipeline is an advanced workflow that leverages the Nuclear MCP Server's 5 powerful tools to perform automated code analysis, quality checks, and security scanning.

## Features

### 🤖 Multi-Agent Analysis
- **Real MCP Server**: Uses the actual Nuclear MCP server (not mocks)
- **Port 8079**: Correct port configuration matching the server
- **5 Real Tools**: 
  1. `websearch` - Web search with 55+ search engines
  2. `premium` - Premium content extraction
  3. `file_search` - Advanced file searching and analysis
  4. `scan` - Deep workspace scanning with Go parallelization
  5. `ai_dataset_trainer` - AI training dataset creation

### 📊 Code Quality Analysis
- Automated formatting checks (`cargo fmt`)
- Clippy linting analysis
- Quality report generation
- PR comments with analysis results

### 🔒 Security Scanning
- Cargo audit for dependency vulnerabilities
- Dependency version checking with cargo-outdated
- Security report generation

## Workflow Triggers

The pipeline runs on:
- **Push** to `main` or `develop` branches
- **Pull requests** to `main` branch
- **Manual dispatch** with optional analysis mode selection:
  - `quick` (default): Fast scan of workspace
  - `full`: Full analysis including web research

## Jobs

### 1. Multi-Agent Analysis

Builds and starts the Nuclear MCP server, then performs:

```bash
# Workspace scanning
POST /mcp/tools/call
{
  "name": "scan",
  "arguments": {"path": "."}
}

# File searching for TODOs/FIXMEs
POST /mcp/tools/call
{
  "name": "file_search",
  "arguments": {
    "path": "src/",
    "query": "TODO|FIXME"
  }
}

# Web search (full mode only)
POST /mcp/tools/call
{
  "name": "websearch",
  "arguments": {
    "query": "rust async best practices 2025"
  }
}
```

**Outputs**:
- `scan_results.json` - Workspace analysis results
- `search_results.json` - File search results
- `websearch_results.json` - Web search results (full mode only)

### 2. Code Quality

Runs after multi-agent analysis:
- Downloads analysis results
- Runs format check
- Runs clippy analysis
- Generates quality report
- Comments on PR with results

**Artifacts**:
- `quality-report/QUALITY_REPORT.md`

### 3. Security Scanning

Runs in parallel with code quality:
- Cargo audit for vulnerabilities
- Dependency version checking
- Security report generation

**Artifacts**:
- `security-report/SECURITY_REPORT.md`

## Configuration

### Environment Variables
- `CARGO_TERM_COLOR`: always
- `RUST_BACKTRACE`: 1
- `NUCLEAR_MODE`: advanced

### Timeouts
- Server startup: 5 seconds
- Tool execution: varies by tool (2-60 seconds)

## Usage

### Manual Trigger

1. Go to Actions tab in GitHub
2. Select "Nuclear Multi-Agent CI/CD Pipeline"
3. Click "Run workflow"
4. Choose analysis mode:
   - **quick**: Fast workspace scan only
   - **full**: Complete analysis with web research

### Automatic Trigger

The workflow runs automatically on:
- Every push to `main` or `develop`
- Every pull request to `main`

## Results

### Analysis Results

The workflow generates several artifacts:

1. **nuclear-analysis-results**: JSON files with raw analysis data
2. **quality-report**: Markdown report with code quality metrics
3. **security-report**: Markdown report with security scan results

### PR Comments

On pull requests, the workflow automatically comments with:
- Code quality analysis
- Issues found (errors, warnings, TODOs)
- Security concerns
- Recommendations

## Troubleshooting

### Server fails to start

**Symptom**: Health check fails at port 8079

**Solution**: 
1. Check if port is already in use
2. Verify binary built successfully
3. Check server logs in workflow output

### Tool calls fail

**Symptom**: JSON-RPC errors or empty responses

**Solution**:
1. Verify tool names are correct (websearch, premium, file_search, scan, ai_dataset_trainer)
2. Check tool arguments match input schema
3. Review MCP server logs

### Build timeout

**Symptom**: Cargo build exceeds time limit

**Solution**:
1. Enable cargo cache (already configured)
2. Consider splitting into multiple jobs
3. Check for dependency issues

## Comparison with Previous Version

### Fixed Issues

1. ✅ **Port**: Changed from 5050 to correct port 8079
2. ✅ **Tools**: Removed non-existent "orchestrate" tool, using real tools
3. ✅ **Features**: Removed invalid `--features advanced` build flag
4. ✅ **Dependencies**: Removed reliance on missing Python scripts
5. ✅ **Complexity**: Simplified from 330 to 268 lines (-19%)

### Improvements

1. **Reliability**: All tools and endpoints actually exist
2. **Testability**: Can be tested locally with real server
3. **Maintainability**: Clearer structure, fewer dependencies
4. **Functionality**: Actually works with the MCP server
5. **Documentation**: Clear explanation of each job

## Development

### Testing Locally

```bash
# Build the server
cargo build --release --bin nuclear-mcp

# Start the server
./target/release/nuclear-mcp &

# Test health endpoint
curl http://localhost:8079/health

# Test tools list
curl -X POST http://localhost:8079/mcp/tools/list \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":"test","method":"tools/list","params":{}}'

# Test scan tool
curl -X POST http://localhost:8079/mcp/tools/call \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": "scan-1",
    "method": "tools/call",
    "params": {
      "name": "scan",
      "arguments": {"path": "."}
    }
  }'
```

### Adding New Analysis

To add new analysis steps:

1. Choose appropriate MCP tool (scan, file_search, websearch, etc.)
2. Add curl call in multi_agent_analysis job
3. Save results to JSON file
4. Upload as artifact
5. Use in subsequent jobs

Example:
```yaml
- name: 🔍 Custom Analysis
  run: |
    curl -X POST http://localhost:8079/mcp/tools/call \
      -H "Content-Type: application/json" \
      -d '{
        "jsonrpc": "2.0",
        "id": "custom-1",
        "method": "tools/call",
        "params": {
          "name": "file_search",
          "arguments": {
            "path": "tests/",
            "query": "assert"
          }
        }
      }' > custom_results.json
```

## Future Enhancements

- [ ] Add benchmark analysis with `ai_dataset_trainer`
- [ ] Integrate with GitHub Issues for automatic issue creation
- [ ] Add performance tracking over time
- [ ] Implement automatic PR creation for fixes
- [ ] Add more detailed analysis modes

## References

- [Nuclear MCP Server Documentation](../../../README.md)
- [MCP Protocol Documentation](../../../API_REFERENCE.md)
- [Tool Specifications](../../../TOOLS.md)
- [Architecture Overview](../../../ARCHITECTURE.md)
