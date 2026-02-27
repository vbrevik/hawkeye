# Incident Report — analytics-pipeline — 2024-12-23

**Date:** 2024-12-23
**Severity:** P2
**Duration:** ~155 minutes
**Service:** analytics-pipeline
**Responders:** Bob Martins, Quinn Murphy

## Summary

Analytics-pipeline experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
155 minutes and affected 57% of traffic.

## Timeline

- **2024-12-23 09:12** — Alerts triggered on DynamoDB metrics
- **2024-12-23 09:18** — Quinn Murphy acknowledged the alert
- **2024-12-23 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-12-23 09:41** — Mitigation applied (rolled back last deployment)
- **2024-12-23 11:47** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 42,377 requests failed
- 487 users affected
- Downstream services impacted: cache-layer, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Bob Martins, Quinn Murphy

## Lessons Learned

We need better integration tests to catch these issues before production.
