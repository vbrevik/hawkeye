# Incident Report — notification-service — 2025-10-10

**Date:** 2025-10-10
**Severity:** P1
**Duration:** ~34 minutes
**Service:** notification-service
**Responders:** Tomas Novak, Bob Martins

## Summary

Notification-service experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
34 minutes and affected 88% of traffic.

## Timeline

- **2025-10-10 09:12** — Alerts triggered on React metrics
- **2025-10-10 09:18** — Tomas Novak acknowledged the alert
- **2025-10-10 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2025-10-10 09:41** — Mitigation applied (rolled back last deployment)
- **2025-10-10 09:46** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 44,955 requests failed
- 260 users affected
- Downstream services impacted: webhook-handler, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for React
- [ ] Schedule blameless post-mortem with Tomas Novak, Bob Martins

## Lessons Learned

We need better alerting coverage to catch these issues before production.
