#!/usr/bin/env python3
"""Repository CI analyzer.

Inspects the single repository workflow policy and prints a local report.
It does not create comments or drive additional workflow automation.
"""
import os
import glob
import yaml

GITHUB_REPO = os.getenv("GITHUB_REPOSITORY") or "Rigohl/nuclear-crawler-hybrid"


def list_workflows():
    files = [p for p in glob.glob('.github/workflows/*.yml') + glob.glob('.github/workflows/*.yaml') if os.path.basename(p) == 'ci.yml']
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
        if problems:
            reports.append((p, problems))
    return reports


if __name__ == '__main__':
    wf_reports = list_workflows()
    if not wf_reports:
        print('No CI workflow issues found in ci.yml.')
    else:
        print('CI workflow issues detected:')
        for path, problems in wf_reports:
            print(f'  - {path}:')
            if isinstance(problems, list):
                for p in problems:
                    print(f'    - {p}')
            else:
                print(f'    - {problems}')
