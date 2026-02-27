# Incident Report — analytics-pipeline — 2024-09-17

**Date:** 2024-09-17
**Severity:** P3
**Duration:** ~177 minutes
**Service:** analytics-pipeline
**Responders:** Jae-won Kim, Gina Torres

## Summary

Analytics-pipeline experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
177 minutes and affected 24% of traffic.

## Timeline

- **2024-09-17 09:12** — Alerts triggered on Axum metrics
- **2024-09-17 09:18** — Gina Torres acknowledged the alert
- **2024-09-17 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-09-17 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-17 11:69** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 12,676 requests failed
- 393 users affected
- Downstream services impacted: data-warehouse, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with Jae-won Kim, Gina Torres

## Lessons Learned

We need better canary deployments to catch these issues before production.
