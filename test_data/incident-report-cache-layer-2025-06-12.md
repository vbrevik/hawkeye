# Incident Report — cache-layer — 2025-06-12

**Date:** 2025-06-12
**Severity:** P3
**Duration:** ~61 minutes
**Service:** cache-layer
**Responders:** Quinn Murphy, Gina Torres, Elena Rossi

## Summary

Cache-layer experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
61 minutes and affected 7% of traffic.

## Timeline

- **2025-06-12 09:12** — Alerts triggered on Redis metrics
- **2025-06-12 09:18** — Quinn Murphy acknowledged the alert
- **2025-06-12 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2025-06-12 09:41** — Mitigation applied (rolled back last deployment)
- **2025-06-12 10:13** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 34,544 requests failed
- 308 users affected
- Downstream services impacted: scheduler, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Redis
- [ ] Schedule blameless post-mortem with Quinn Murphy, Gina Torres

## Lessons Learned

We need better alerting coverage to catch these issues before production.
