# Incident Report — event-bus — 2023-01-20

**Date:** 2023-01-20
**Severity:** P3
**Duration:** ~197 minutes
**Service:** event-bus
**Responders:** Henrik Larsen, Nadia Kovač, Sofia Andersen

## Summary

Event-bus experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
197 minutes and affected 56% of traffic.

## Timeline

- **2023-01-20 09:12** — Alerts triggered on SQLite metrics
- **2023-01-20 09:18** — Sofia Andersen acknowledged the alert
- **2023-01-20 09:25** — Root cause identified: retry storm after upstream timeout
- **2023-01-20 09:41** — Mitigation applied (rolled back last deployment)
- **2023-01-20 12:29** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 19,737 requests failed
- 167 users affected
- Downstream services impacted: search-service, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for SQLite
- [ ] Schedule blameless post-mortem with Henrik Larsen, Nadia Kovač

## Lessons Learned

We need better alerting coverage to catch these issues before production.
