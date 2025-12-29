## Description
Please include a summary of the changes and the related issue. Include relevant motivation and context.

Fixes # (issue)

## Type of change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Refactoring (no functional changes)
- [ ] Performance improvement
- [ ] CI/CD changes
- [ ] FFI Integration (Go/Zig/Nim changes)

## MCP Tools Affected
- [ ] websearch
- [ ] deepweb_search
- [ ] premium_content_scraper
- [ ] file_search
- [ ] Core infrastructure (cache, rate limiter, storage)
- [ ] MCP protocol implementation
- [ ] None (documentation, CI/CD, etc.)

## Checklist
- [ ] My code follows the project's style guidelines (`cargo fmt`)
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings (`cargo clippy -- -D warnings`)
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally (`cargo test`)
- [ ] Any dependent changes have been merged and published
- [ ] FFI libraries compile successfully (if modified)
- [ ] Docker image builds successfully (if applicable)

## Testing
Describe the tests that you ran to verify your changes:

- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual testing
- [ ] Docker testing

**Test Configuration**:
- OS: 
- Rust version: 
- Docker version (if applicable): 

## FFI Changes (if applicable)
- [ ] Go FFI updated (`go/src/`)
- [ ] Zig SIMD updated (`zig/src/`)
- [ ] Nim HTML parser updated (`nim/src/`)
- [ ] Libraries recompiled and tested
- [ ] Cross-platform compatibility verified

## Performance Impact
- [ ] No performance impact
- [ ] Performance improved (provide benchmarks)
- [ ] Potential performance regression (justify if necessary)

**Benchmarks** (if applicable):
```
Before: 
After: 
```

## Security Considerations
- [ ] No security impact
- [ ] Security vulnerability fixed (link to private advisory)
- [ ] New dependencies audited (`cargo audit`)
- [ ] Input validation added/updated
- [ ] Authentication/authorization changes

## Documentation
- [ ] README.md updated
- [ ] ARCHITECTURE.md updated
- [ ] DEPLOYMENT.md updated
- [ ] CONTRIBUTING.md updated
- [ ] MCP_TOOLS_REFERENCE.md updated
- [ ] Inline code documentation added
- [ ] No documentation changes needed

## Screenshots/Logs (if appropriate)
Add screenshots, logs, or MCP tool output to help explain your changes.

```
[Paste relevant logs or tool output here]
```

## Additional Notes
Add any other context about the pull request here.

---

## For Maintainers
- [ ] Version bump required (semver)
- [ ] Release notes drafted
- [ ] Breaking changes documented
- [ ] Migration guide provided (if breaking change)
