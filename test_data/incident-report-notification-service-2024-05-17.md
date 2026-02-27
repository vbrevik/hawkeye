# Incident Report — notification-service — 2024-05-17

**Date:** 2024-05-17
**Severity:** P2
**Duration:** ~240 minutes
**Service:** notification-service
**Responders:** Elena Rossi, Nadia Kovač

## Summary

Notification-service experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
240 minutes and affected 57% of traffic.

## Timeline

- **2024-05-17 09:12** — Alerts triggered on Axum metrics
- **2024-05-17 09:18** — Nadia Kovač acknowledged the alert
- **2024-05-17 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2024-05-17 09:41** — Mitigation applied (rolled back last deployment)
- **2024-05-17 13:12** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 44,808 requests failed
- 186 users affected
- Downstream services impacted: auth-service, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with Elena Rossi, Nadia Kovač

## Lessons Learned

We need better canary deployments to catch these issues before production.
