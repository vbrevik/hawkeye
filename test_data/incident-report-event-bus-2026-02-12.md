# Incident Report — event-bus — 2026-02-12

**Date:** 2026-02-12
**Severity:** P2
**Duration:** ~146 minutes
**Service:** event-bus
**Responders:** Frank Müller, Mohamed Al-Rashid

## Summary

Event-bus experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
146 minutes and affected 67% of traffic.

## Timeline

- **2026-02-12 09:12** — Alerts triggered on Celery metrics
- **2026-02-12 09:18** — Frank Müller acknowledged the alert
- **2026-02-12 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2026-02-12 09:41** — Mitigation applied (rolled back last deployment)
- **2026-02-12 11:38** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 38,452 requests failed
- 156 users affected
- Downstream services impacted: event-bus, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Celery
- [ ] Schedule blameless post-mortem with Frank Müller, Mohamed Al-Rashid

## Lessons Learned

We need better integration tests to catch these issues before production.
