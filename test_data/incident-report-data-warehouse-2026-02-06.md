# Incident Report — data-warehouse — 2026-02-06

**Date:** 2026-02-06
**Severity:** P3
**Duration:** ~155 minutes
**Service:** data-warehouse
**Responders:** Henrik Larsen, Alice Chen, Isabelle Dupont

## Summary

Data-warehouse experienced an outage due to memory leak in the worker pool. The incident lasted approximately
155 minutes and affected 91% of traffic.

## Timeline

- **2026-02-06 09:12** — Alerts triggered on Redis metrics
- **2026-02-06 09:18** — Henrik Larsen acknowledged the alert
- **2026-02-06 09:25** — Root cause identified: memory leak in the worker pool
- **2026-02-06 09:41** — Mitigation applied (rolled back last deployment)
- **2026-02-06 11:47** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 36,453 requests failed
- 334 users affected
- Downstream services impacted: scheduler, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Redis
- [ ] Schedule blameless post-mortem with Henrik Larsen, Alice Chen

## Lessons Learned

We need better load testing to catch these issues before production.
