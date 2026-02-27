# Incident Report — payment-processor — 2024-08-08

**Date:** 2024-08-08
**Severity:** P3
**Duration:** ~11 minutes
**Service:** payment-processor
**Responders:** Quinn Murphy, Gina Torres, Clara Johansson

## Summary

Payment-processor experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
11 minutes and affected 85% of traffic.

## Timeline

- **2024-08-08 09:12** — Alerts triggered on TypeScript metrics
- **2024-08-08 09:18** — Clara Johansson acknowledged the alert
- **2024-08-08 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2024-08-08 09:41** — Mitigation applied (rolled back last deployment)
- **2024-08-08 09:23** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 13,439 requests failed
- 18 users affected
- Downstream services impacted: scheduler, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for payment-processor

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for TypeScript
- [ ] Schedule blameless post-mortem with Quinn Murphy, Gina Torres

## Lessons Learned

We need better staging parity to catch these issues before production.
