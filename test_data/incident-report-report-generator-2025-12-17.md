# Incident Report — report-generator — 2025-12-17

**Date:** 2025-12-17
**Severity:** P2
**Duration:** ~197 minutes
**Service:** report-generator
**Responders:** Ravi Sharma, Mohamed Al-Rashid, David Park, Elena Rossi

## Summary

Report-generator experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
197 minutes and affected 79% of traffic.

## Timeline

- **2025-12-17 09:12** — Alerts triggered on Docker metrics
- **2025-12-17 09:18** — Ravi Sharma acknowledged the alert
- **2025-12-17 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-12-17 09:41** — Mitigation applied (rolled back last deployment)
- **2025-12-17 12:29** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 30,084 requests failed
- 336 users affected
- Downstream services impacted: notification-service, search-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Docker
- [ ] Schedule blameless post-mortem with Ravi Sharma, Mohamed Al-Rashid

## Lessons Learned

We need better canary deployments to catch these issues before production.
