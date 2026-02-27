# Incident Report — notification-service — 2025-03-27

**Date:** 2025-03-27
**Severity:** P3
**Duration:** ~54 minutes
**Service:** notification-service
**Responders:** Gina Torres, Tomas Novak

## Summary

Notification-service experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
54 minutes and affected 89% of traffic.

## Timeline

- **2025-03-27 09:12** — Alerts triggered on FastAPI metrics
- **2025-03-27 09:18** — Tomas Novak acknowledged the alert
- **2025-03-27 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-03-27 09:41** — Mitigation applied (rolled back last deployment)
- **2025-03-27 09:66** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 36,027 requests failed
- 442 users affected
- Downstream services impacted: scheduler, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Gina Torres, Tomas Novak

## Lessons Learned

We need better integration tests to catch these issues before production.
