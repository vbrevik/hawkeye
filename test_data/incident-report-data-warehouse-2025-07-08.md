# Incident Report — data-warehouse — 2025-07-08

**Date:** 2025-07-08
**Severity:** P3
**Duration:** ~199 minutes
**Service:** data-warehouse
**Responders:** Sofia Andersen, Alice Chen, Henrik Larsen, Tomas Novak

## Summary

Data-warehouse experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
199 minutes and affected 20% of traffic.

## Timeline

- **2025-07-08 09:12** — Alerts triggered on Vault metrics
- **2025-07-08 09:18** — Henrik Larsen acknowledged the alert
- **2025-07-08 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-07-08 09:41** — Mitigation applied (rolled back last deployment)
- **2025-07-08 12:31** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 48,459 requests failed
- 220 users affected
- Downstream services impacted: scheduler, payment-processor

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Vault
- [ ] Schedule blameless post-mortem with Sofia Andersen, Alice Chen

## Lessons Learned

We need better alerting coverage to catch these issues before production.
