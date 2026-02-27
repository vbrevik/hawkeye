# Incident Report — api-gateway — 2023-10-10

**Date:** 2023-10-10
**Severity:** P2
**Duration:** ~78 minutes
**Service:** api-gateway
**Responders:** Henrik Larsen, Priya Patel

## Summary

Api-gateway experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
78 minutes and affected 69% of traffic.

## Timeline

- **2023-10-10 09:12** — Alerts triggered on TypeScript metrics
- **2023-10-10 09:18** — Henrik Larsen acknowledged the alert
- **2023-10-10 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2023-10-10 09:41** — Mitigation applied (rolled back last deployment)
- **2023-10-10 10:30** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 17,533 requests failed
- 191 users affected
- Downstream services impacted: report-generator, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for TypeScript
- [ ] Schedule blameless post-mortem with Henrik Larsen, Priya Patel

## Lessons Learned

We need better integration tests to catch these issues before production.
