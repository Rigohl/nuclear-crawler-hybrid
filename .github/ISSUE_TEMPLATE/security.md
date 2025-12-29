---
name: Security Vulnerability Report
about: Report a security vulnerability in Nuclear Crawler Hybrid (Private)
title: "[SECURITY] "
labels: security
assignees: ''

---

⚠️ **IMPORTANT: Do NOT report security vulnerabilities in public issues!**

For security vulnerabilities, please use one of the following methods:

1. **GitHub Security Advisory** (Preferred)
   - Go to: https://github.com/Rigohl/nuclear-crawler-hybrid/security/advisories
   - Click "Report a vulnerability"
   - Fill out the private advisory form

2. **Email** (Alternative)
   - Send details to: security@nuclear-crawler.dev
   - Use PGP encryption if possible

---

## Security Vulnerability Information

**Vulnerability Type**
- [ ] Remote Code Execution (RCE)
- [ ] SQL Injection
- [ ] Cross-Site Scripting (XSS)
- [ ] Authentication bypass
- [ ] Authorization bypass
- [ ] Sensitive data exposure
- [ ] Server-Side Request Forgery (SSRF)
- [ ] Dependency vulnerability
- [ ] FFI memory safety issue
- [ ] Other: _____________

**Affected Component**
- [ ] MCP Server core
- [ ] WebSearch tool
- [ ] DeepWeb Search tool
- [ ] Premium Content Scraper
- [ ] File Search tool
- [ ] Go FFI integration
- [ ] Zig SIMD integration
- [ ] Nim HTML parser
- [ ] Docker image
- [ ] Dependencies

**Severity Assessment**
- [ ] Critical (9.0-10.0 CVSS)
- [ ] High (7.0-8.9 CVSS)
- [ ] Medium (4.0-6.9 CVSS)
- [ ] Low (0.1-3.9 CVSS)

**Description**
[Provide a clear description of the vulnerability]

**Steps to Reproduce**
1. 
2. 
3. 

**Proof of Concept**
[Include PoC code, commands, or screenshots if available]

**Impact**
[Describe the potential impact of this vulnerability]

**Affected Versions**
- Version: [e.g., 0.1.0, all versions]
- Deployment: [e.g., Docker, native, both]

**Suggested Fix**
[If you have suggestions for fixing the vulnerability]

**References**
- CVE ID (if assigned): 
- Related issues: 
- External links: 

---

## Security Best Practices Reminder

When reporting:
- ✅ Use private disclosure methods
- ✅ Include detailed reproduction steps
- ✅ Provide version information
- ✅ Allow time for patch development
- ❌ Do NOT disclose publicly until patched
- ❌ Do NOT exploit the vulnerability

Thank you for helping keep Nuclear Crawler Hybrid secure!
