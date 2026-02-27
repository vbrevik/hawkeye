# Incident Report — notification-service — 2024-08-31

**Date:** 2024-08-31
**Severity:** P3
**Duration:** ~116 minutes
**Service:** notification-service
**Responders:** Priya Patel, Ravi Sharma, Frank Müller

## Summary

Notification-service experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
116 minutes and affected 50% of traffic.

## Timeline

- **2024-08-31 09:12** — Alerts triggered on TypeScript metrics
- **2024-08-31 09:18** — Frank Müller acknowledged the alert
- **2024-08-31 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2024-08-31 09:41** — Mitigation applied (rolled back last deployment)
- **2024-08-31 10:68** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 25,602 requests failed
- 422 users affected
- Downstream services impacted: auth-service, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for TypeScript
- [ ] Schedule blameless post-mortem with Priya Patel, Ravi Sharma

## Lessons Learned

We need better alerting coverage to catch these issues before production.
