# Incident Report — event-bus — 2024-09-17

**Date:** 2024-09-17
**Severity:** P3
**Duration:** ~55 minutes
**Service:** event-bus
**Responders:** Tomas Novak, Clara Johansson, Isabelle Dupont, Elena Rossi

## Summary

Event-bus experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
55 minutes and affected 38% of traffic.

## Timeline

- **2024-09-17 09:12** — Alerts triggered on ArgoCD metrics
- **2024-09-17 09:18** — Tomas Novak acknowledged the alert
- **2024-09-17 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2024-09-17 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-17 09:67** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 1,320 requests failed
- 435 users affected
- Downstream services impacted: cache-layer, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for ArgoCD
- [ ] Schedule blameless post-mortem with Tomas Novak, Clara Johansson

## Lessons Learned

We need better staging parity to catch these issues before production.
