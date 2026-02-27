# Incident Report — scheduler — 2024-10-26

**Date:** 2024-10-26
**Severity:** P3
**Duration:** ~17 minutes
**Service:** scheduler
**Responders:** Elena Rossi, Frank Müller

## Summary

Scheduler experienced an outage due to race condition during concurrent writes. The incident lasted approximately
17 minutes and affected 91% of traffic.

## Timeline

- **2024-10-26 09:12** — Alerts triggered on Helm metrics
- **2024-10-26 09:18** — Elena Rossi acknowledged the alert
- **2024-10-26 09:25** — Root cause identified: race condition during concurrent writes
- **2024-10-26 09:41** — Mitigation applied (rolled back last deployment)
- **2024-10-26 09:29** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 29,139 requests failed
- 339 users affected
- Downstream services impacted: data-warehouse, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Helm
- [ ] Schedule blameless post-mortem with Elena Rossi, Frank Müller

## Lessons Learned

We need better alerting coverage to catch these issues before production.
