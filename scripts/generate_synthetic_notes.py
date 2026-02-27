#!/usr/bin/env python3
"""
Generate realistic synthetic markdown notes to fill test_data/ to 1000 files.
Covers domains: engineering, meetings, research, devops, product, architecture.

Usage:
    python3 scripts/generate_synthetic_notes.py [OUTPUT_DIR] [TARGET_COUNT]
"""

import os
import random
import sys
from pathlib import Path
from datetime import datetime, timedelta

OUTPUT_DIR = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("test_data")
TARGET_COUNT = int(sys.argv[2]) if len(sys.argv) > 2 else 1000

random.seed(42)

# --- Domain data ---

PEOPLE = [
    "Alice Chen", "Bob Martins", "Clara Johansson", "David Park",
    "Elena Rossi", "Frank Müller", "Gina Torres", "Henrik Larsen",
    "Isabelle Dupont", "Jae-won Kim", "Kofi Mensah", "Laura Bianchi",
    "Mohamed Al-Rashid", "Nadia Kovač", "Oscar Lindberg", "Priya Patel",
    "Quinn Murphy", "Ravi Sharma", "Sofia Andersen", "Tomas Novak",
]

SERVICES = [
    "auth-service", "api-gateway", "notification-service", "payment-processor",
    "user-service", "analytics-pipeline", "data-warehouse", "event-bus",
    "cache-layer", "search-service", "media-uploader", "report-generator",
    "scheduler", "webhook-handler", "audit-logger",
]

TECHNOLOGIES = [
    "Kubernetes", "PostgreSQL", "Redis", "Kafka", "Rust", "Go", "TypeScript",
    "React", "Terraform", "Docker", "Prometheus", "Grafana", "Elasticsearch",
    "gRPC", "GraphQL", "Nginx", "Vault", "ArgoCD", "Helm", "S3",
    "DynamoDB", "RabbitMQ", "Celery", "FastAPI", "Axum", "SQLite",
]

PROJECTS = [
    "Project Phoenix", "Atlas Platform", "Nexus API", "Glacier Storage",
    "Beacon Analytics", "Lighthouse CMS", "Forge CI/CD", "Stratos Deploy",
    "Meridian Data", "Apex Security", "Pulse Monitoring", "Core Refactor Q1",
    "Migration to K8s", "Auth Overhaul", "Search Rewrite",
]

ISSUES = [
    "memory leak in the worker pool",
    "race condition during concurrent writes",
    "cache invalidation not propagating across regions",
    "slow query on the user lookup table (missing index)",
    "token expiry edge case when clock skew > 30s",
    "retry storm after upstream timeout",
    "goroutine leak in the WebSocket handler",
    "flaky tests in the integration suite",
    "SSL certificate not renewing automatically",
    "disk I/O bottleneck during bulk import",
]

ACTIONS = [
    "refactor the authentication middleware",
    "add rate limiting to the public API",
    "migrate the legacy monolith to microservices",
    "implement circuit breakers for downstream calls",
    "write runbooks for the on-call team",
    "set up alerting for P99 latency",
    "review and rotate all secrets in Vault",
    "add structured logging with trace IDs",
    "benchmark the new storage backend",
    "document the deployment process",
]

DECISIONS = [
    "We will use Rust for the new service due to memory safety and performance.",
    "PostgreSQL chosen over MongoDB — relational model fits our query patterns better.",
    "Decided to go with a pull-based deployment model using ArgoCD.",
    "Agreed to sunset the legacy Python service by end of Q2.",
    "Will use Redis for session storage — simple and battle-tested.",
    "Chose gRPC over REST for the internal service mesh.",
    "Team agreed on a 2-week sprint cadence going forward.",
    "Feature flags will be managed via LaunchDarkly.",
    "We will require code review from 2 engineers before merging.",
    "Adopted conventional commits across all repositories.",
]


def random_date(start_year=2023, end_year=2026):
    start = datetime(start_year, 1, 1)
    end = datetime(end_year, 2, 27)
    delta = end - start
    return (start + timedelta(days=random.randint(0, delta.days))).strftime("%Y-%m-%d")


def pick(lst, n=1):
    return random.sample(lst, min(n, len(lst)))


def make_meeting_note():
    date = random_date()
    project = random.choice(PROJECTS)
    attendees = pick(PEOPLE, random.randint(2, 5))
    decisions = pick(DECISIONS, random.randint(1, 3))
    actions = pick(ACTIONS, random.randint(2, 4))
    services = pick(SERVICES, random.randint(1, 3))
    tech = pick(TECHNOLOGIES, random.randint(1, 4))

    title = f"Meeting Notes — {project} — {date}"
    content = f"""# {title}

**Date:** {date}
**Attendees:** {", ".join(attendees)}
**Project:** {project}

## Agenda

- Status update on {services[0]}
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of {project}. {random.choice(attendees)} raised concerns about {random.choice(ISSUES)}.
{random.choice(attendees)} explained that this was related to the recent changes in {services[0]}.

We discussed migrating to {tech[0]} for better performance. {random.choice(attendees)} had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between {services[0]} and {services[-1] if len(services) > 1 else services[0]} was identified as a risk.
{random.choice(attendees)} will own this investigation.

## Decisions

{"".join(f"- {d}" + chr(10) for d in decisions)}
## Action Items

{"".join(f"- [ ] {a.capitalize()} — **{random.choice(attendees)}** — Due {random_date(2026, 2026)}" + chr(10) for a in actions)}
## Notes

Stack: {", ".join(tech)}
Services involved: {", ".join(services)}
"""
    return title, content


def make_incident_report():
    date = random_date()
    service = random.choice(SERVICES)
    tech = pick(TECHNOLOGIES, 2)
    issue = random.choice(ISSUES)
    responders = pick(PEOPLE, random.randint(2, 4))

    severity = random.choice(["P1", "P2", "P3"])
    duration = random.randint(5, 240)
    title = f"Incident Report — {service} — {date}"

    content = f"""# {title}

**Date:** {date}
**Severity:** {severity}
**Duration:** ~{duration} minutes
**Service:** {service}
**Responders:** {", ".join(responders)}

## Summary

{service.capitalize()} experienced an outage due to {issue}. The incident lasted approximately
{duration} minutes and affected {random.randint(5, 95)}% of traffic.

## Timeline

- **{date} 09:12** — Alerts triggered on {tech[0]} metrics
- **{date} 09:18** — {random.choice(responders)} acknowledged the alert
- **{date} 09:25** — Root cause identified: {issue}
- **{date} 09:41** — Mitigation applied (rolled back last deployment)
- **{date} {9 + duration // 60:02d}:{12 + duration % 60:02d}** — Service fully restored

## Root Cause

The root cause was {issue}. This was introduced in the latest release when
{random.choice(ACTIONS)}. The change was not caught in staging because the
load pattern was different.

## Impact

- {random.randint(100, 50000):,} requests failed
- {random.randint(1, 500)} users affected
- Downstream services impacted: {", ".join(pick(SERVICES, 2))}

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for {issue[:40]}
3. Updated runbook for {service}

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for {tech[0]}
- [ ] Schedule blameless post-mortem with {", ".join(responders[:2])}

## Lessons Learned

We need better {random.choice(["staging parity", "canary deployments", "load testing",
"integration tests", "alerting coverage"])} to catch these issues before production.
"""
    return title, content


def make_design_doc():
    project = random.choice(PROJECTS)
    tech = pick(TECHNOLOGIES, random.randint(3, 6))
    date = random_date()
    author = random.choice(PEOPLE)
    services = pick(SERVICES, random.randint(2, 4))

    title = f"Design Doc — {project}"
    content = f"""# Design Doc: {project}

**Author:** {author}
**Date:** {date}
**Status:** Draft

## Overview

This document describes the design for {project}. The goal is to {random.choice(ACTIONS)}
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, {random.choice(SERVICES)} suffers from {random.choice(ISSUES)}.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will {random.choice(ACTIONS)} using {tech[0]} as the primary technology.

### Architecture

```
[Client] → [{services[0]}] → [{services[1]}] → [Database]
                ↓
          [{tech[0]}]
```

The system will consist of:
- **{services[0]}**: Handles incoming requests and authentication
- **{services[1] if len(services) > 1 else services[0]}**: Core business logic
- **{tech[0]}**: Caching and state management

### Data Flow

1. Client sends request to {services[0]}
2. {services[0]} validates the token using {tech[1] if len(tech) > 1 else tech[0]}
3. Request is forwarded to {services[1] if len(services) > 1 else services[0]}
4. Response is cached in {tech[0]} with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| {tech[0]} | Fast, well-supported | Higher operational complexity |
| {tech[1] if len(tech) > 1 else "Alternative"} | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle {random.choice(ISSUES)}?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
"""
    return title, content


def make_research_note():
    tech = random.choice(TECHNOLOGIES)
    date = random_date()
    author = random.choice(PEOPLE)
    title = f"Research Notes — {tech} Evaluation"

    content = f"""# {title}

**Author:** {author}
**Date:** {date}

## Purpose

Evaluating {tech} for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for {random.choice(SERVICES)}.

## Key Findings

### Performance

Initial benchmarks show {tech} handles approximately {random.randint(5000, 100000):,} req/s
under our expected load profile. This is {random.choice(["significantly better than",
"comparable to", "slightly worse than"])} our current setup.

Memory usage is {random.choice(["lower", "higher", "similar"])} — roughly
{random.randint(50, 500)}MB under load compared to our current {random.randint(100, 600)}MB.

### Operational Complexity

{tech} requires {random.choice(["minimal", "moderate", "significant"])} operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The {tech} community is {random.choice(["active", "growing", "mature and stable"])}.
Documentation quality is {random.choice(["excellent", "good", "adequate"])}.
Last major release: {random_date(2025, 2026)}.

### Integration

Integration with our existing {random.choice(SERVICES)} is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

{random.choice(["Adopt", "Adopt with caveats", "Evaluate further", "Do not adopt"])} {tech}.

**Rationale:** {random.choice(DECISIONS)}

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with {random.choice(PEOPLE)}
- [ ] Prototype integration with {random.choice(SERVICES)}
- [ ] Get sign-off from {random.choice(PEOPLE)}
"""
    return title, content


def make_runbook():
    service = random.choice(SERVICES)
    tech = pick(TECHNOLOGIES, 2)
    title = f"Runbook — {service}"

    content = f"""# Runbook: {service}

**Last updated:** {random_date()}
**Owner:** {random.choice(PEOPLE)}
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for {service}.
The service runs on {tech[0]} and uses {tech[1] if len(tech) > 1 else tech[0]} for storage.

## Health Check

```bash
curl -s https://internal.example.com/{service}/health | jq .
```

Expected response:
```json
{{"status": "ok", "version": "1.4.2", "uptime": 123456}}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `{service}-overview`
2. Inspect slow query logs: `kubectl logs -l app={service} | grep "slow_query"`
3. Check downstream dependencies: {random.choice(SERVICES)}
4. If needed, scale up: `kubectl scale deployment {service} --replicas=6`

### Issue 2: {random.choice(ISSUES).capitalize()}

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/{service}`
2. Rollback if needed: `kubectl rollout undo deployment/{service}`
3. Page {random.choice(PEOPLE)} if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app={service}`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: {random.choice(PEOPLE)}
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app={service} --tail=100 -f

# Describe pods
kubectl describe pods -l app={service}

# Restart deployment
kubectl rollout restart deployment/{service}
```
"""
    return title, content


GENERATORS = [
    (make_meeting_note, 0.25),
    (make_incident_report, 0.20),
    (make_design_doc, 0.25),
    (make_research_note, 0.15),
    (make_runbook, 0.15),
]


def pick_generator():
    r = random.random()
    cumulative = 0
    for gen, weight in GENERATORS:
        cumulative += weight
        if r < cumulative:
            return gen
    return GENERATORS[0][0]


def slugify(s):
    import re
    s = s.lower().strip()
    s = re.sub(r"[^\w\s-]", "", s)
    s = re.sub(r"[\s_-]+", "-", s)
    return s[:80].strip("-")


def main():
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    existing = set(p.stem for p in OUTPUT_DIR.glob("*.md"))
    current = len(existing)
    needed = TARGET_COUNT - current

    if needed <= 0:
        print(f"Already have {current} files. Done.")
        return

    print(f"Have {current} files, generating {needed} more...")

    generated = 0
    attempts = 0

    while generated < needed:
        attempts += 1
        gen = pick_generator()
        title, content = gen()
        slug = slugify(title)

        # Add counter to handle duplicate titles
        final_slug = slug
        counter = 1
        while final_slug in existing:
            final_slug = f"{slug}-{counter}"
            counter += 1

        path = OUTPUT_DIR / f"{final_slug}.md"
        path.write_text(content, encoding="utf-8")
        existing.add(final_slug)
        generated += 1

        if generated % 100 == 0:
            print(f"  {generated}/{needed} generated...")

    total = len(list(OUTPUT_DIR.glob("*.md")))
    print(f"Done! {total} markdown files in {OUTPUT_DIR}/")


if __name__ == "__main__":
    main()
