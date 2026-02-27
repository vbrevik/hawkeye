# Incident Report — auth-service — 2023-10-18

**Date:** 2023-10-18
**Severity:** P1
**Duration:** ~74 minutes
**Service:** auth-service
**Responders:** Oscar Lindberg, David Park, Alice Chen, Priya Patel

## Summary

Auth-service experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
74 minutes and affected 40% of traffic.

## Timeline

- **2023-10-18 09:12** — Alerts triggered on React metrics
- **2023-10-18 09:18** — Priya Patel acknowledged the alert
- **2023-10-18 09:25** — Root cause identified: flaky tests in the integration suite
- **2023-10-18 09:41** — Mitigation applied (rolled back last deployment)
- **2023-10-18 10:26** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 5,637 requests failed
- 31 users affected
- Downstream services impacted: user-service, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for React
- [ ] Schedule blameless post-mortem with Oscar Lindberg, David Park

## Lessons Learned

We need better alerting coverage to catch these issues before production.
