# Incident Report — scheduler — 2025-10-26

**Date:** 2025-10-26
**Severity:** P2
**Duration:** ~116 minutes
**Service:** scheduler
**Responders:** Tomas Novak, Jae-won Kim

## Summary

Scheduler experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
116 minutes and affected 30% of traffic.

## Timeline

- **2025-10-26 09:12** — Alerts triggered on Axum metrics
- **2025-10-26 09:18** — Jae-won Kim acknowledged the alert
- **2025-10-26 09:25** — Root cause identified: retry storm after upstream timeout
- **2025-10-26 09:41** — Mitigation applied (rolled back last deployment)
- **2025-10-26 10:68** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 35,216 requests failed
- 110 users affected
- Downstream services impacted: user-service, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with Tomas Novak, Jae-won Kim

## Lessons Learned

We need better staging parity to catch these issues before production.
