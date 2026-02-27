# Incident Report — notification-service — 2023-01-28

**Date:** 2023-01-28
**Severity:** P1
**Duration:** ~14 minutes
**Service:** notification-service
**Responders:** Gina Torres, Oscar Lindberg

## Summary

Notification-service experienced an outage due to race condition during concurrent writes. The incident lasted approximately
14 minutes and affected 59% of traffic.

## Timeline

- **2023-01-28 09:12** — Alerts triggered on Vault metrics
- **2023-01-28 09:18** — Oscar Lindberg acknowledged the alert
- **2023-01-28 09:25** — Root cause identified: race condition during concurrent writes
- **2023-01-28 09:41** — Mitigation applied (rolled back last deployment)
- **2023-01-28 09:26** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 41,136 requests failed
- 412 users affected
- Downstream services impacted: cache-layer, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Vault
- [ ] Schedule blameless post-mortem with Gina Torres, Oscar Lindberg

## Lessons Learned

We need better canary deployments to catch these issues before production.
