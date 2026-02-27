# Incident Report — auth-service — 2023-11-20

**Date:** 2023-11-20
**Severity:** P3
**Duration:** ~171 minutes
**Service:** auth-service
**Responders:** Frank Müller, Clara Johansson

## Summary

Auth-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
171 minutes and affected 80% of traffic.

## Timeline

- **2023-11-20 09:12** — Alerts triggered on Nginx metrics
- **2023-11-20 09:18** — Clara Johansson acknowledged the alert
- **2023-11-20 09:25** — Root cause identified: memory leak in the worker pool
- **2023-11-20 09:41** — Mitigation applied (rolled back last deployment)
- **2023-11-20 11:63** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 45,549 requests failed
- 414 users affected
- Downstream services impacted: scheduler, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Nginx
- [ ] Schedule blameless post-mortem with Frank Müller, Clara Johansson

## Lessons Learned

We need better integration tests to catch these issues before production.
