# Incident Report — webhook-handler — 2023-01-17

**Date:** 2023-01-17
**Severity:** P1
**Duration:** ~107 minutes
**Service:** webhook-handler
**Responders:** Laura Bianchi, Mohamed Al-Rashid, Tomas Novak

## Summary

Webhook-handler experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
107 minutes and affected 11% of traffic.

## Timeline

- **2023-01-17 09:12** — Alerts triggered on Kafka metrics
- **2023-01-17 09:18** — Tomas Novak acknowledged the alert
- **2023-01-17 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2023-01-17 09:41** — Mitigation applied (rolled back last deployment)
- **2023-01-17 10:59** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 12,856 requests failed
- 186 users affected
- Downstream services impacted: cache-layer, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kafka
- [ ] Schedule blameless post-mortem with Laura Bianchi, Mohamed Al-Rashid

## Lessons Learned

We need better staging parity to catch these issues before production.
