# Incident Report — scheduler — 2024-11-27

**Date:** 2024-11-27
**Severity:** P3
**Duration:** ~17 minutes
**Service:** scheduler
**Responders:** Isabelle Dupont, Elena Rossi, Quinn Murphy, David Park

## Summary

Scheduler experienced an outage due to memory leak in the worker pool. The incident lasted approximately
17 minutes and affected 61% of traffic.

## Timeline

- **2024-11-27 09:12** — Alerts triggered on Grafana metrics
- **2024-11-27 09:18** — David Park acknowledged the alert
- **2024-11-27 09:25** — Root cause identified: memory leak in the worker pool
- **2024-11-27 09:41** — Mitigation applied (rolled back last deployment)
- **2024-11-27 09:29** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 7,039 requests failed
- 401 users affected
- Downstream services impacted: search-service, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Grafana
- [ ] Schedule blameless post-mortem with Isabelle Dupont, Elena Rossi

## Lessons Learned

We need better staging parity to catch these issues before production.
