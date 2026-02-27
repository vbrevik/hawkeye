# Incident Report — report-generator — 2024-05-20

**Date:** 2024-05-20
**Severity:** P2
**Duration:** ~6 minutes
**Service:** report-generator
**Responders:** Gina Torres, Oscar Lindberg, David Park, Elena Rossi

## Summary

Report-generator experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
6 minutes and affected 55% of traffic.

## Timeline

- **2024-05-20 09:12** — Alerts triggered on DynamoDB metrics
- **2024-05-20 09:18** — David Park acknowledged the alert
- **2024-05-20 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2024-05-20 09:41** — Mitigation applied (rolled back last deployment)
- **2024-05-20 09:18** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 24,996 requests failed
- 412 users affected
- Downstream services impacted: webhook-handler, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Gina Torres, Oscar Lindberg

## Lessons Learned

We need better integration tests to catch these issues before production.
