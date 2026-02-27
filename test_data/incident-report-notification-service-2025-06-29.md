# Incident Report — notification-service — 2025-06-29

**Date:** 2025-06-29
**Severity:** P3
**Duration:** ~116 minutes
**Service:** notification-service
**Responders:** Elena Rossi, Jae-won Kim

## Summary

Notification-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
116 minutes and affected 70% of traffic.

## Timeline

- **2025-06-29 09:12** — Alerts triggered on React metrics
- **2025-06-29 09:18** — Jae-won Kim acknowledged the alert
- **2025-06-29 09:25** — Root cause identified: memory leak in the worker pool
- **2025-06-29 09:41** — Mitigation applied (rolled back last deployment)
- **2025-06-29 10:68** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 38,351 requests failed
- 454 users affected
- Downstream services impacted: cache-layer, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for React
- [ ] Schedule blameless post-mortem with Elena Rossi, Jae-won Kim

## Lessons Learned

We need better canary deployments to catch these issues before production.
