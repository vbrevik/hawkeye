# Incident Report — notification-service — 2025-08-28

**Date:** 2025-08-28
**Severity:** P3
**Duration:** ~189 minutes
**Service:** notification-service
**Responders:** Kofi Mensah, Sofia Andersen, Nadia Kovač, Elena Rossi

## Summary

Notification-service experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
189 minutes and affected 56% of traffic.

## Timeline

- **2025-08-28 09:12** — Alerts triggered on PostgreSQL metrics
- **2025-08-28 09:18** — Sofia Andersen acknowledged the alert
- **2025-08-28 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-08-28 09:41** — Mitigation applied (rolled back last deployment)
- **2025-08-28 12:21** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 7,885 requests failed
- 124 users affected
- Downstream services impacted: data-warehouse, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for PostgreSQL
- [ ] Schedule blameless post-mortem with Kofi Mensah, Sofia Andersen

## Lessons Learned

We need better load testing to catch these issues before production.
