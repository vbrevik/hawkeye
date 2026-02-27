# Incident Report — cache-layer — 2024-05-15

**Date:** 2024-05-15
**Severity:** P3
**Duration:** ~227 minutes
**Service:** cache-layer
**Responders:** Frank Müller, Clara Johansson

## Summary

Cache-layer experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
227 minutes and affected 53% of traffic.

## Timeline

- **2024-05-15 09:12** — Alerts triggered on gRPC metrics
- **2024-05-15 09:18** — Frank Müller acknowledged the alert
- **2024-05-15 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2024-05-15 09:41** — Mitigation applied (rolled back last deployment)
- **2024-05-15 12:59** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 38,317 requests failed
- 74 users affected
- Downstream services impacted: payment-processor, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for gRPC
- [ ] Schedule blameless post-mortem with Frank Müller, Clara Johansson

## Lessons Learned

We need better load testing to catch these issues before production.
