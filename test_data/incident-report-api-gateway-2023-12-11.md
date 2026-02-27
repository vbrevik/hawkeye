# Incident Report — api-gateway — 2023-12-11

**Date:** 2023-12-11
**Severity:** P3
**Duration:** ~214 minutes
**Service:** api-gateway
**Responders:** Quinn Murphy, Gina Torres, Laura Bianchi, Ravi Sharma

## Summary

Api-gateway experienced an outage due to race condition during concurrent writes. The incident lasted approximately
214 minutes and affected 87% of traffic.

## Timeline

- **2023-12-11 09:12** — Alerts triggered on Vault metrics
- **2023-12-11 09:18** — Gina Torres acknowledged the alert
- **2023-12-11 09:25** — Root cause identified: race condition during concurrent writes
- **2023-12-11 09:41** — Mitigation applied (rolled back last deployment)
- **2023-12-11 12:46** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,837 requests failed
- 75 users affected
- Downstream services impacted: user-service, payment-processor

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Vault
- [ ] Schedule blameless post-mortem with Quinn Murphy, Gina Torres

## Lessons Learned

We need better canary deployments to catch these issues before production.
