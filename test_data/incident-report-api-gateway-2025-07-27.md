# Incident Report — api-gateway — 2025-07-27

**Date:** 2025-07-27
**Severity:** P3
**Duration:** ~113 minutes
**Service:** api-gateway
**Responders:** Mohamed Al-Rashid, Gina Torres, Tomas Novak

## Summary

Api-gateway experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
113 minutes and affected 81% of traffic.

## Timeline

- **2025-07-27 09:12** — Alerts triggered on Kubernetes metrics
- **2025-07-27 09:18** — Gina Torres acknowledged the alert
- **2025-07-27 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2025-07-27 09:41** — Mitigation applied (rolled back last deployment)
- **2025-07-27 10:65** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,344 requests failed
- 286 users affected
- Downstream services impacted: auth-service, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Mohamed Al-Rashid, Gina Torres

## Lessons Learned

We need better load testing to catch these issues before production.
