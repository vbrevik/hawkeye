# Incident Report — audit-logger — 2023-10-17

**Date:** 2023-10-17
**Severity:** P2
**Duration:** ~148 minutes
**Service:** audit-logger
**Responders:** Tomas Novak, Clara Johansson

## Summary

Audit-logger experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
148 minutes and affected 53% of traffic.

## Timeline

- **2023-10-17 09:12** — Alerts triggered on Grafana metrics
- **2023-10-17 09:18** — Clara Johansson acknowledged the alert
- **2023-10-17 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2023-10-17 09:41** — Mitigation applied (rolled back last deployment)
- **2023-10-17 11:40** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 44,016 requests failed
- 360 users affected
- Downstream services impacted: webhook-handler, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for audit-logger

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Grafana
- [ ] Schedule blameless post-mortem with Tomas Novak, Clara Johansson

## Lessons Learned

We need better alerting coverage to catch these issues before production.
