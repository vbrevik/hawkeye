# Incident Report — scheduler — 2023-10-14

**Date:** 2023-10-14
**Severity:** P1
**Duration:** ~219 minutes
**Service:** scheduler
**Responders:** Laura Bianchi, Bob Martins, David Park, Nadia Kovač

## Summary

Scheduler experienced an outage due to race condition during concurrent writes. The incident lasted approximately
219 minutes and affected 14% of traffic.

## Timeline

- **2023-10-14 09:12** — Alerts triggered on Celery metrics
- **2023-10-14 09:18** — David Park acknowledged the alert
- **2023-10-14 09:25** — Root cause identified: race condition during concurrent writes
- **2023-10-14 09:41** — Mitigation applied (rolled back last deployment)
- **2023-10-14 12:51** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 40,330 requests failed
- 489 users affected
- Downstream services impacted: search-service, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Celery
- [ ] Schedule blameless post-mortem with Laura Bianchi, Bob Martins

## Lessons Learned

We need better load testing to catch these issues before production.
