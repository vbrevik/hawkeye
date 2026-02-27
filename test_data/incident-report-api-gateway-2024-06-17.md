# Incident Report — api-gateway — 2024-06-17

**Date:** 2024-06-17
**Severity:** P2
**Duration:** ~236 minutes
**Service:** api-gateway
**Responders:** Isabelle Dupont, Laura Bianchi, Nadia Kovač

## Summary

Api-gateway experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
236 minutes and affected 38% of traffic.

## Timeline

- **2024-06-17 09:12** — Alerts triggered on Prometheus metrics
- **2024-06-17 09:18** — Nadia Kovač acknowledged the alert
- **2024-06-17 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2024-06-17 09:41** — Mitigation applied (rolled back last deployment)
- **2024-06-17 12:68** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 33,980 requests failed
- 161 users affected
- Downstream services impacted: data-warehouse, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Prometheus
- [ ] Schedule blameless post-mortem with Isabelle Dupont, Laura Bianchi

## Lessons Learned

We need better load testing to catch these issues before production.
