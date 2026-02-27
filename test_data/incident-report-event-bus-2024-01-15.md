# Incident Report — event-bus — 2024-01-15

**Date:** 2024-01-15
**Severity:** P3
**Duration:** ~63 minutes
**Service:** event-bus
**Responders:** Mohamed Al-Rashid, Nadia Kovač, Gina Torres

## Summary

Event-bus experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
63 minutes and affected 57% of traffic.

## Timeline

- **2024-01-15 09:12** — Alerts triggered on RabbitMQ metrics
- **2024-01-15 09:18** — Mohamed Al-Rashid acknowledged the alert
- **2024-01-15 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2024-01-15 09:41** — Mitigation applied (rolled back last deployment)
- **2024-01-15 10:15** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 8,201 requests failed
- 239 users affected
- Downstream services impacted: api-gateway, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for RabbitMQ
- [ ] Schedule blameless post-mortem with Mohamed Al-Rashid, Nadia Kovač

## Lessons Learned

We need better canary deployments to catch these issues before production.
