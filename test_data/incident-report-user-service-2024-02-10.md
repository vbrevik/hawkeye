# Incident Report — user-service — 2024-02-10

**Date:** 2024-02-10
**Severity:** P1
**Duration:** ~130 minutes
**Service:** user-service
**Responders:** Mohamed Al-Rashid, Clara Johansson, Priya Patel

## Summary

User-service experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
130 minutes and affected 18% of traffic.

## Timeline

- **2024-02-10 09:12** — Alerts triggered on DynamoDB metrics
- **2024-02-10 09:18** — Clara Johansson acknowledged the alert
- **2024-02-10 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2024-02-10 09:41** — Mitigation applied (rolled back last deployment)
- **2024-02-10 11:22** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 31,826 requests failed
- 313 users affected
- Downstream services impacted: data-warehouse, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Mohamed Al-Rashid, Clara Johansson

## Lessons Learned

We need better integration tests to catch these issues before production.
