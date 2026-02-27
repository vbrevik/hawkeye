# Incident Report — data-warehouse — 2025-06-07

**Date:** 2025-06-07
**Severity:** P3
**Duration:** ~33 minutes
**Service:** data-warehouse
**Responders:** Isabelle Dupont, David Park

## Summary

Data-warehouse experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
33 minutes and affected 92% of traffic.

## Timeline

- **2025-06-07 09:12** — Alerts triggered on Kubernetes metrics
- **2025-06-07 09:18** — David Park acknowledged the alert
- **2025-06-07 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-06-07 09:41** — Mitigation applied (rolled back last deployment)
- **2025-06-07 09:45** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 21,684 requests failed
- 328 users affected
- Downstream services impacted: api-gateway, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Isabelle Dupont, David Park

## Lessons Learned

We need better load testing to catch these issues before production.
