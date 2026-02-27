# Incident Report — auth-service — 2025-08-13

**Date:** 2025-08-13
**Severity:** P3
**Duration:** ~25 minutes
**Service:** auth-service
**Responders:** Nadia Kovač, Bob Martins

## Summary

Auth-service experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
25 minutes and affected 40% of traffic.

## Timeline

- **2025-08-13 09:12** — Alerts triggered on Grafana metrics
- **2025-08-13 09:18** — Bob Martins acknowledged the alert
- **2025-08-13 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-08-13 09:41** — Mitigation applied (rolled back last deployment)
- **2025-08-13 09:37** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 28,964 requests failed
- 93 users affected
- Downstream services impacted: user-service, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Grafana
- [ ] Schedule blameless post-mortem with Nadia Kovač, Bob Martins

## Lessons Learned

We need better load testing to catch these issues before production.
