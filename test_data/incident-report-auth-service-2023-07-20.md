# Incident Report — auth-service — 2023-07-20

**Date:** 2023-07-20
**Severity:** P2
**Duration:** ~50 minutes
**Service:** auth-service
**Responders:** Jae-won Kim, Gina Torres

## Summary

Auth-service experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
50 minutes and affected 81% of traffic.

## Timeline

- **2023-07-20 09:12** — Alerts triggered on DynamoDB metrics
- **2023-07-20 09:18** — Gina Torres acknowledged the alert
- **2023-07-20 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2023-07-20 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-20 09:62** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 39,909 requests failed
- 253 users affected
- Downstream services impacted: cache-layer, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Jae-won Kim, Gina Torres

## Lessons Learned

We need better staging parity to catch these issues before production.
