# Incident Report — api-gateway — 2025-01-29

**Date:** 2025-01-29
**Severity:** P3
**Duration:** ~157 minutes
**Service:** api-gateway
**Responders:** Tomas Novak, Quinn Murphy, Jae-won Kim

## Summary

Api-gateway experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
157 minutes and affected 12% of traffic.

## Timeline

- **2025-01-29 09:12** — Alerts triggered on S3 metrics
- **2025-01-29 09:18** — Tomas Novak acknowledged the alert
- **2025-01-29 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2025-01-29 09:41** — Mitigation applied (rolled back last deployment)
- **2025-01-29 11:49** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 28,868 requests failed
- 6 users affected
- Downstream services impacted: payment-processor, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for S3
- [ ] Schedule blameless post-mortem with Tomas Novak, Quinn Murphy

## Lessons Learned

We need better alerting coverage to catch these issues before production.
