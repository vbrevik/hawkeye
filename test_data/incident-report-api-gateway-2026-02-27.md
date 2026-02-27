# Incident Report — api-gateway — 2026-02-27

**Date:** 2026-02-27
**Severity:** P3
**Duration:** ~48 minutes
**Service:** api-gateway
**Responders:** Clara Johansson, David Park

## Summary

Api-gateway experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
48 minutes and affected 31% of traffic.

## Timeline

- **2026-02-27 09:12** — Alerts triggered on DynamoDB metrics
- **2026-02-27 09:18** — David Park acknowledged the alert
- **2026-02-27 09:25** — Root cause identified: retry storm after upstream timeout
- **2026-02-27 09:41** — Mitigation applied (rolled back last deployment)
- **2026-02-27 09:60** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 16,481 requests failed
- 43 users affected
- Downstream services impacted: notification-service, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Clara Johansson, David Park

## Lessons Learned

We need better alerting coverage to catch these issues before production.
