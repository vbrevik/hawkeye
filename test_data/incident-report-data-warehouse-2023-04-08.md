# Incident Report — data-warehouse — 2023-04-08

**Date:** 2023-04-08
**Severity:** P2
**Duration:** ~219 minutes
**Service:** data-warehouse
**Responders:** Gina Torres, Isabelle Dupont

## Summary

Data-warehouse experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
219 minutes and affected 85% of traffic.

## Timeline

- **2023-04-08 09:12** — Alerts triggered on Kafka metrics
- **2023-04-08 09:18** — Gina Torres acknowledged the alert
- **2023-04-08 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2023-04-08 09:41** — Mitigation applied (rolled back last deployment)
- **2023-04-08 12:51** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 48,897 requests failed
- 172 users affected
- Downstream services impacted: user-service, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kafka
- [ ] Schedule blameless post-mortem with Gina Torres, Isabelle Dupont

## Lessons Learned

We need better load testing to catch these issues before production.
