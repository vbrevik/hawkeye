# Incident Report — user-service — 2025-07-18

**Date:** 2025-07-18
**Severity:** P1
**Duration:** ~195 minutes
**Service:** user-service
**Responders:** Gina Torres, Kofi Mensah

## Summary

User-service experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
195 minutes and affected 73% of traffic.

## Timeline

- **2025-07-18 09:12** — Alerts triggered on Docker metrics
- **2025-07-18 09:18** — Gina Torres acknowledged the alert
- **2025-07-18 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-07-18 09:41** — Mitigation applied (rolled back last deployment)
- **2025-07-18 12:27** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 14,280 requests failed
- 379 users affected
- Downstream services impacted: event-bus, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Docker
- [ ] Schedule blameless post-mortem with Gina Torres, Kofi Mensah

## Lessons Learned

We need better alerting coverage to catch these issues before production.
