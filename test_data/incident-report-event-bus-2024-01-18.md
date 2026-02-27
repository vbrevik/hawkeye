# Incident Report — event-bus — 2024-01-18

**Date:** 2024-01-18
**Severity:** P3
**Duration:** ~20 minutes
**Service:** event-bus
**Responders:** Isabelle Dupont, Henrik Larsen, Tomas Novak

## Summary

Event-bus experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
20 minutes and affected 24% of traffic.

## Timeline

- **2024-01-18 09:12** — Alerts triggered on PostgreSQL metrics
- **2024-01-18 09:18** — Tomas Novak acknowledged the alert
- **2024-01-18 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-01-18 09:41** — Mitigation applied (rolled back last deployment)
- **2024-01-18 09:32** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 5,716 requests failed
- 343 users affected
- Downstream services impacted: analytics-pipeline, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for PostgreSQL
- [ ] Schedule blameless post-mortem with Isabelle Dupont, Henrik Larsen

## Lessons Learned

We need better alerting coverage to catch these issues before production.
