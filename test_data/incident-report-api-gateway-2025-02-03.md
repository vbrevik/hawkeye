# Incident Report — api-gateway — 2025-02-03

**Date:** 2025-02-03
**Severity:** P2
**Duration:** ~72 minutes
**Service:** api-gateway
**Responders:** David Park, Jae-won Kim, Sofia Andersen, Priya Patel

## Summary

Api-gateway experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
72 minutes and affected 32% of traffic.

## Timeline

- **2025-02-03 09:12** — Alerts triggered on Prometheus metrics
- **2025-02-03 09:18** — David Park acknowledged the alert
- **2025-02-03 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-02-03 09:41** — Mitigation applied (rolled back last deployment)
- **2025-02-03 10:24** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 25,896 requests failed
- 112 users affected
- Downstream services impacted: audit-logger, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Prometheus
- [ ] Schedule blameless post-mortem with David Park, Jae-won Kim

## Lessons Learned

We need better alerting coverage to catch these issues before production.
