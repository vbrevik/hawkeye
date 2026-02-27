# Incident Report — user-service — 2023-09-05

**Date:** 2023-09-05
**Severity:** P1
**Duration:** ~117 minutes
**Service:** user-service
**Responders:** Laura Bianchi, Ravi Sharma

## Summary

User-service experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
117 minutes and affected 36% of traffic.

## Timeline

- **2023-09-05 09:12** — Alerts triggered on Rust metrics
- **2023-09-05 09:18** — Laura Bianchi acknowledged the alert
- **2023-09-05 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2023-09-05 09:41** — Mitigation applied (rolled back last deployment)
- **2023-09-05 10:69** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,651 requests failed
- 406 users affected
- Downstream services impacted: webhook-handler, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Rust
- [ ] Schedule blameless post-mortem with Laura Bianchi, Ravi Sharma

## Lessons Learned

We need better alerting coverage to catch these issues before production.
