# Incident Report — event-bus — 2026-01-17

**Date:** 2026-01-17
**Severity:** P2
**Duration:** ~202 minutes
**Service:** event-bus
**Responders:** Alice Chen, David Park, Henrik Larsen

## Summary

Event-bus experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
202 minutes and affected 61% of traffic.

## Timeline

- **2026-01-17 09:12** — Alerts triggered on Go metrics
- **2026-01-17 09:18** — Alice Chen acknowledged the alert
- **2026-01-17 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2026-01-17 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-17 12:34** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 13,153 requests failed
- 236 users affected
- Downstream services impacted: report-generator, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Alice Chen, David Park

## Lessons Learned

We need better staging parity to catch these issues before production.
