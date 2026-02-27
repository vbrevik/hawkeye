# Incident Report — notification-service — 2025-12-11

**Date:** 2025-12-11
**Severity:** P1
**Duration:** ~33 minutes
**Service:** notification-service
**Responders:** Henrik Larsen, Clara Johansson, Nadia Kovač

## Summary

Notification-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
33 minutes and affected 65% of traffic.

## Timeline

- **2025-12-11 09:12** — Alerts triggered on Go metrics
- **2025-12-11 09:18** — Clara Johansson acknowledged the alert
- **2025-12-11 09:25** — Root cause identified: memory leak in the worker pool
- **2025-12-11 09:41** — Mitigation applied (rolled back last deployment)
- **2025-12-11 09:45** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 35,274 requests failed
- 226 users affected
- Downstream services impacted: notification-service, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Henrik Larsen, Clara Johansson

## Lessons Learned

We need better alerting coverage to catch these issues before production.
