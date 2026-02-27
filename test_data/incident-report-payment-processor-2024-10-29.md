# Incident Report — payment-processor — 2024-10-29

**Date:** 2024-10-29
**Severity:** P3
**Duration:** ~110 minutes
**Service:** payment-processor
**Responders:** David Park, Henrik Larsen

## Summary

Payment-processor experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
110 minutes and affected 30% of traffic.

## Timeline

- **2024-10-29 09:12** — Alerts triggered on Axum metrics
- **2024-10-29 09:18** — Henrik Larsen acknowledged the alert
- **2024-10-29 09:25** — Root cause identified: retry storm after upstream timeout
- **2024-10-29 09:41** — Mitigation applied (rolled back last deployment)
- **2024-10-29 10:62** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 4,918 requests failed
- 242 users affected
- Downstream services impacted: search-service, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for payment-processor

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with David Park, Henrik Larsen

## Lessons Learned

We need better staging parity to catch these issues before production.
