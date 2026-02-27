# Incident Report — notification-service — 2026-01-16

**Date:** 2026-01-16
**Severity:** P1
**Duration:** ~95 minutes
**Service:** notification-service
**Responders:** Alice Chen, Bob Martins, Jae-won Kim, Mohamed Al-Rashid

## Summary

Notification-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
95 minutes and affected 73% of traffic.

## Timeline

- **2026-01-16 09:12** — Alerts triggered on DynamoDB metrics
- **2026-01-16 09:18** — Alice Chen acknowledged the alert
- **2026-01-16 09:25** — Root cause identified: memory leak in the worker pool
- **2026-01-16 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-16 10:47** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 48,500 requests failed
- 261 users affected
- Downstream services impacted: cache-layer, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Alice Chen, Bob Martins

## Lessons Learned

We need better canary deployments to catch these issues before production.
