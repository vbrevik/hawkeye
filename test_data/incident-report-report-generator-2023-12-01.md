# Incident Report — report-generator — 2023-12-01

**Date:** 2023-12-01
**Severity:** P2
**Duration:** ~165 minutes
**Service:** report-generator
**Responders:** Henrik Larsen, Isabelle Dupont, Clara Johansson

## Summary

Report-generator experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
165 minutes and affected 35% of traffic.

## Timeline

- **2023-12-01 09:12** — Alerts triggered on FastAPI metrics
- **2023-12-01 09:18** — Clara Johansson acknowledged the alert
- **2023-12-01 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2023-12-01 09:41** — Mitigation applied (rolled back last deployment)
- **2023-12-01 11:57** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 45,577 requests failed
- 351 users affected
- Downstream services impacted: user-service, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Henrik Larsen, Isabelle Dupont

## Lessons Learned

We need better alerting coverage to catch these issues before production.
