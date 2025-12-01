# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.5.x   | :white_check_mark: |
| < 0.5   | :x:                |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability, please follow these steps:

### 🔒 Private Disclosure

**Do NOT open a public issue for security vulnerabilities.**

Instead, please report security vulnerabilities by:

1. **Email**: Send details to the repository owner via GitHub
2. **GitHub Security Advisories**: Use the "Security" tab to report privately

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 24-48 hours
  - High: 7 days
  - Medium: 30 days
  - Low: Next release

## Security Measures

### Code Security

- ✅ Automated dependency audits with `cargo-audit`
- ✅ Static analysis with Clippy
- ✅ Memory safety guaranteed by Rust
- ✅ No unsafe code in critical paths

### Runtime Security

- ✅ TLS 1.3 enforced for all connections
- ✅ No plaintext credential storage
- ✅ Request rate limiting
- ✅ Input validation and sanitization

### Container Security

- ✅ Non-root user execution
- ✅ Minimal base image (distroless)
- ✅ No secrets in images
- ✅ Read-only filesystem where possible

## Security Updates

Security updates are released as patch versions (e.g., 0.5.1) and announced through:

- GitHub Releases
- Security Advisories

## Best Practices for Users

1. **Keep Updated**: Always use the latest version
2. **Environment Variables**: Use env vars for sensitive config
3. **Network Security**: Run behind a reverse proxy in production
4. **Monitoring**: Enable logging and monitor for anomalies

---

Thank you for helping keep Nuclear Crawler secure! 🛡️
