# Incident Report — search-service — 2023-10-25

**Date:** 2023-10-25
**Severity:** P1
**Duration:** ~167 minutes
**Service:** search-service
**Responders:** Bob Martins, Gina Torres, Frank Müller, Alice Chen

## Summary

Search-service experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
167 minutes and affected 9% of traffic.

## Timeline

- **2023-10-25 09:12** — Alerts triggered on FastAPI metrics
- **2023-10-25 09:18** — Gina Torres acknowledged the alert
- **2023-10-25 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2023-10-25 09:41** — Mitigation applied (rolled back last deployment)
- **2023-10-25 11:59** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 26,102 requests failed
- 166 users affected
- Downstream services impacted: auth-service, search-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for search-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Bob Martins, Gina Torres

## Lessons Learned

We need better integration tests to catch these issues before production.
