# Incident Report — report-generator — 2024-09-19

**Date:** 2024-09-19
**Severity:** P3
**Duration:** ~114 minutes
**Service:** report-generator
**Responders:** Nadia Kovač, Priya Patel, Bob Martins

## Summary

Report-generator experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
114 minutes and affected 87% of traffic.

## Timeline

- **2024-09-19 09:12** — Alerts triggered on DynamoDB metrics
- **2024-09-19 09:18** — Nadia Kovač acknowledged the alert
- **2024-09-19 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2024-09-19 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-19 10:66** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 7,312 requests failed
- 138 users affected
- Downstream services impacted: analytics-pipeline, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Nadia Kovač, Priya Patel

## Lessons Learned

We need better load testing to catch these issues before production.
