# Incident Report — audit-logger — 2024-03-08

**Date:** 2024-03-08
**Severity:** P3
**Duration:** ~122 minutes
**Service:** audit-logger
**Responders:** Priya Patel, Mohamed Al-Rashid

## Summary

Audit-logger experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
122 minutes and affected 23% of traffic.

## Timeline

- **2024-03-08 09:12** — Alerts triggered on Helm metrics
- **2024-03-08 09:18** — Mohamed Al-Rashid acknowledged the alert
- **2024-03-08 09:25** — Root cause identified: retry storm after upstream timeout
- **2024-03-08 09:41** — Mitigation applied (rolled back last deployment)
- **2024-03-08 11:14** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 16,262 requests failed
- 382 users affected
- Downstream services impacted: cache-layer, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for audit-logger

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Helm
- [ ] Schedule blameless post-mortem with Priya Patel, Mohamed Al-Rashid

## Lessons Learned

We need better load testing to catch these issues before production.
