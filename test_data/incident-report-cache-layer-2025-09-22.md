# Incident Report — cache-layer — 2025-09-22

**Date:** 2025-09-22
**Severity:** P3
**Duration:** ~99 minutes
**Service:** cache-layer
**Responders:** Ravi Sharma, Tomas Novak, Jae-won Kim, Oscar Lindberg

## Summary

Cache-layer experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
99 minutes and affected 91% of traffic.

## Timeline

- **2025-09-22 09:12** — Alerts triggered on Vault metrics
- **2025-09-22 09:18** — Ravi Sharma acknowledged the alert
- **2025-09-22 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-09-22 09:41** — Mitigation applied (rolled back last deployment)
- **2025-09-22 10:51** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 30,107 requests failed
- 45 users affected
- Downstream services impacted: api-gateway, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Vault
- [ ] Schedule blameless post-mortem with Ravi Sharma, Tomas Novak

## Lessons Learned

We need better canary deployments to catch these issues before production.
