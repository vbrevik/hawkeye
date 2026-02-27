# Incident Report — scheduler — 2023-07-30

**Date:** 2023-07-30
**Severity:** P1
**Duration:** ~143 minutes
**Service:** scheduler
**Responders:** Oscar Lindberg, Jae-won Kim

## Summary

Scheduler experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
143 minutes and affected 83% of traffic.

## Timeline

- **2023-07-30 09:12** — Alerts triggered on DynamoDB metrics
- **2023-07-30 09:18** — Jae-won Kim acknowledged the alert
- **2023-07-30 09:25** — Root cause identified: flaky tests in the integration suite
- **2023-07-30 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-30 11:35** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 4,713 requests failed
- 451 users affected
- Downstream services impacted: scheduler, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Jae-won Kim

## Lessons Learned

We need better load testing to catch these issues before production.
