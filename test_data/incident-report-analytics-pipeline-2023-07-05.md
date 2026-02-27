# Incident Report — analytics-pipeline — 2023-07-05

**Date:** 2023-07-05
**Severity:** P3
**Duration:** ~15 minutes
**Service:** analytics-pipeline
**Responders:** Jae-won Kim, Gina Torres, Frank Müller

## Summary

Analytics-pipeline experienced an outage due to race condition during concurrent writes. The incident lasted approximately
15 minutes and affected 54% of traffic.

## Timeline

- **2023-07-05 09:12** — Alerts triggered on ArgoCD metrics
- **2023-07-05 09:18** — Frank Müller acknowledged the alert
- **2023-07-05 09:25** — Root cause identified: race condition during concurrent writes
- **2023-07-05 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-05 09:27** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 19,686 requests failed
- 357 users affected
- Downstream services impacted: data-warehouse, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for ArgoCD
- [ ] Schedule blameless post-mortem with Jae-won Kim, Gina Torres

## Lessons Learned

We need better load testing to catch these issues before production.
