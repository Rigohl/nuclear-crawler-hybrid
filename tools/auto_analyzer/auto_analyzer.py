#!/usr/bin/env python3
"""
Auto-analyzer for workflows and PRs.
- Runs in GitHub Actions (uses GITHUB_TOKEN)
- Scans `.github/workflows/*.yml` for common issues
- Posts a single summarizing comment per PR (avoids duplicates)
"""
import os
import glob
import yaml
import requests

GITHUB_REPO = os.getenv("GITHUB_REPOSITORY") or "Rigohl/nuclear-crawler-hybrid"
GITHUB_TOKEN = os.getenv("GITHUB_TOKEN")
API_BASE = "https://api.github.com"
HEADERS = {"Authorization": f"token {GITHUB_TOKEN}", "Accept": "application/vnd.github.v3+json"}

SIGNATURE = "<!-- auto-analyzer-report -->"


def list_workflows():
    files = glob.glob('.github/workflows/*.yml') + glob.glob('.github/workflows/*.yaml')
    reports = []
    for p in files:
        try:
            with open(p, 'r', encoding='utf-8') as f:
                data = yaml.safe_load(f) or {}
        except Exception as e:
            reports.append((p, f'parse_error: {e}'))
            continue
        problems = []
        if not data.get('concurrency'):
            problems.append('missing `concurrency` to avoid duplicate runs')
        if not data.get('permissions'):
            problems.append('missing `permissions` block; consider limiting tokens')
        jobs = data.get('jobs', {})
        for job_name, job in jobs.items():
            if isinstance(job, dict):
                steps = job.get('steps') or []
                uses_checkout = any('uses' in s and 'actions/checkout' in str(s.get('uses')) for s in steps if isinstance(s, dict))
                if not uses_checkout:
                    problems.append(f'job `{job_name}` has no checkout step')
                # check for caching or retry hints
                has_retry = any('retry' in str(s) for s in steps)
                if not has_retry:
                    problems.append(f'job `{job_name}` has no retry mechanism')
        if problems:
            reports.append((p, problems))
    return reports


def list_open_prs(owner, repo):
    url = f"{API_BASE}/repos/{owner}/{repo}/pulls?state=open&per_page=100"
    r = requests.get(url, headers=HEADERS, timeout=30)
    r.raise_for_status()
    return r.json()


def already_commented(owner, repo, pr_number):
    url = f"{API_BASE}/repos/{owner}/{repo}/issues/{pr_number}/comments"
    r = requests.get(url, headers=HEADERS, timeout=30)
    r.raise_for_status()
    for c in r.json():
        if SIGNATURE in c.get('body', ''):
            return True
    return False


def post_pr_comment(owner, repo, pr_number, body):
    url = f"{API_BASE}/repos/{owner}/{repo}/issues/{pr_number}/comments"
    payload = {"body": body}
    r = requests.post(url, json=payload, headers=HEADERS, timeout=30)
    r.raise_for_status()
    return r.json()


if __name__ == '__main__':
    owner, repo = GITHUB_REPO.split('/') if '/' in GITHUB_REPO else ("Rigohl", "nuclear-crawler-hybrid")
    if not GITHUB_TOKEN:
        print('GITHUB_TOKEN not set; exiting (no post)')
        exit(0)

    wf_reports = list_workflows()
    summary_lines = []
    if not wf_reports:
        summary_lines.append('- No workflow issues found.')
    else:
        summary_lines.append('- Workflow issues detected:')
        for path, problems in wf_reports:
            summary_lines.append(f'  - {path}:')
            if isinstance(problems, list):
                for p in problems:
                    summary_lines.append(f'    - {p}')
            else:
                summary_lines.append(f'    - {problems}')

    prs = list_open_prs(owner, repo)
    if not prs:
        print('No open PRs found.')
    for pr in prs:
        num = pr['number']
        title = pr['title']
        if already_commented(owner, repo, num):
            print(f'PR #{num} already has an auto-analyzer comment; skipping.')
            continue
        body = f"{SIGNATURE}\n## Auto-Analyzer Report for PR #{num} - {title}\n\n"
        body += "\n".join(summary_lines)
        body += "\n\n**Suggestions:**\n- Add `concurrency` blocks to long-running workflows.\n- Add `permissions` to reduce token surface.\n- Use `actions/checkout@v4` and `fetch-depth: 0` when building.\n- Add transient-failure retry steps or `continue-on-error`+auto-retry job.\n\n_This comment was posted automatically by the repository auto-analyzer._"
        try:
            post_pr_comment(owner, repo, num, body)
            print(f'Posted report to PR #{num}')
        except Exception as e:
            print(f'Failed to post comment to PR #{num}: {e}')
