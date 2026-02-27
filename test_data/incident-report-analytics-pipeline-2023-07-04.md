# Incident Report — analytics-pipeline — 2023-07-04

**Date:** 2023-07-04
**Severity:** P2
**Duration:** ~164 minutes
**Service:** analytics-pipeline
**Responders:** Jae-won Kim, Clara Johansson

## Summary

Analytics-pipeline experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
164 minutes and affected 45% of traffic.

## Timeline

- **2023-07-04 09:12** — Alerts triggered on Grafana metrics
- **2023-07-04 09:18** — Clara Johansson acknowledged the alert
- **2023-07-04 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2023-07-04 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-04 11:56** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 22,291 requests failed
- 361 users affected
- Downstream services impacted: notification-service, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Grafana
- [ ] Schedule blameless post-mortem with Jae-won Kim, Clara Johansson

## Lessons Learned

We need better load testing to catch these issues before production.
