# Incident Report — user-service — 2023-08-01

**Date:** 2023-08-01
**Severity:** P3
**Duration:** ~42 minutes
**Service:** user-service
**Responders:** Clara Johansson, Alice Chen, Elena Rossi

## Summary

User-service experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
42 minutes and affected 45% of traffic.

## Timeline

- **2023-08-01 09:12** — Alerts triggered on Nginx metrics
- **2023-08-01 09:18** — Clara Johansson acknowledged the alert
- **2023-08-01 09:25** — Root cause identified: retry storm after upstream timeout
- **2023-08-01 09:41** — Mitigation applied (rolled back last deployment)
- **2023-08-01 09:54** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 29,374 requests failed
- 370 users affected
- Downstream services impacted: scheduler, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Nginx
- [ ] Schedule blameless post-mortem with Clara Johansson, Alice Chen

## Lessons Learned

We need better integration tests to catch these issues before production.
