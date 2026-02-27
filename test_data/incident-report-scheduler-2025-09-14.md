# Incident Report — scheduler — 2025-09-14

**Date:** 2025-09-14
**Severity:** P2
**Duration:** ~127 minutes
**Service:** scheduler
**Responders:** Sofia Andersen, David Park, Nadia Kovač, Elena Rossi

## Summary

Scheduler experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
127 minutes and affected 34% of traffic.

## Timeline

- **2025-09-14 09:12** — Alerts triggered on Axum metrics
- **2025-09-14 09:18** — Sofia Andersen acknowledged the alert
- **2025-09-14 09:25** — Root cause identified: retry storm after upstream timeout
- **2025-09-14 09:41** — Mitigation applied (rolled back last deployment)
- **2025-09-14 11:19** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 39,309 requests failed
- 449 users affected
- Downstream services impacted: cache-layer, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with Sofia Andersen, David Park

## Lessons Learned

We need better load testing to catch these issues before production.
