# Incident Report — event-bus — 2023-09-28

**Date:** 2023-09-28
**Severity:** P2
**Duration:** ~99 minutes
**Service:** event-bus
**Responders:** David Park, Tomas Novak

## Summary

Event-bus experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
99 minutes and affected 43% of traffic.

## Timeline

- **2023-09-28 09:12** — Alerts triggered on Redis metrics
- **2023-09-28 09:18** — David Park acknowledged the alert
- **2023-09-28 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2023-09-28 09:41** — Mitigation applied (rolled back last deployment)
- **2023-09-28 10:51** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 49,458 requests failed
- 448 users affected
- Downstream services impacted: notification-service, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Redis
- [ ] Schedule blameless post-mortem with David Park, Tomas Novak

## Lessons Learned

We need better canary deployments to catch these issues before production.
