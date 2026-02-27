# Incident Report — auth-service — 2023-09-24

**Date:** 2023-09-24
**Severity:** P1
**Duration:** ~114 minutes
**Service:** auth-service
**Responders:** Mohamed Al-Rashid, Bob Martins

## Summary

Auth-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
114 minutes and affected 55% of traffic.

## Timeline

- **2023-09-24 09:12** — Alerts triggered on Nginx metrics
- **2023-09-24 09:18** — Mohamed Al-Rashid acknowledged the alert
- **2023-09-24 09:25** — Root cause identified: memory leak in the worker pool
- **2023-09-24 09:41** — Mitigation applied (rolled back last deployment)
- **2023-09-24 10:66** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 38,946 requests failed
- 323 users affected
- Downstream services impacted: webhook-handler, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Nginx
- [ ] Schedule blameless post-mortem with Mohamed Al-Rashid, Bob Martins

## Lessons Learned

We need better integration tests to catch these issues before production.
