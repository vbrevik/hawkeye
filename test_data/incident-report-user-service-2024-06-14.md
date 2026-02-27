# Incident Report — user-service — 2024-06-14

**Date:** 2024-06-14
**Severity:** P2
**Duration:** ~106 minutes
**Service:** user-service
**Responders:** David Park, Nadia Kovač

## Summary

User-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
106 minutes and affected 67% of traffic.

## Timeline

- **2024-06-14 09:12** — Alerts triggered on S3 metrics
- **2024-06-14 09:18** — David Park acknowledged the alert
- **2024-06-14 09:25** — Root cause identified: memory leak in the worker pool
- **2024-06-14 09:41** — Mitigation applied (rolled back last deployment)
- **2024-06-14 10:58** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 4,050 requests failed
- 468 users affected
- Downstream services impacted: cache-layer, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for S3
- [ ] Schedule blameless post-mortem with David Park, Nadia Kovač

## Lessons Learned

We need better staging parity to catch these issues before production.
