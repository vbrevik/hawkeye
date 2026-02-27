# Incident Report — webhook-handler — 2023-09-19

**Date:** 2023-09-19
**Severity:** P1
**Duration:** ~225 minutes
**Service:** webhook-handler
**Responders:** Bob Martins, Alice Chen, Ravi Sharma, Laura Bianchi

## Summary

Webhook-handler experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
225 minutes and affected 41% of traffic.

## Timeline

- **2023-09-19 09:12** — Alerts triggered on Docker metrics
- **2023-09-19 09:18** — Bob Martins acknowledged the alert
- **2023-09-19 09:25** — Root cause identified: retry storm after upstream timeout
- **2023-09-19 09:41** — Mitigation applied (rolled back last deployment)
- **2023-09-19 12:57** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 28,392 requests failed
- 360 users affected
- Downstream services impacted: search-service, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Docker
- [ ] Schedule blameless post-mortem with Bob Martins, Alice Chen

## Lessons Learned

We need better load testing to catch these issues before production.
