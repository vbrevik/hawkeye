# Incident Report — data-warehouse — 2026-01-17

**Date:** 2026-01-17
**Severity:** P2
**Duration:** ~118 minutes
**Service:** data-warehouse
**Responders:** Bob Martins, Jae-won Kim, Clara Johansson

## Summary

Data-warehouse experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
118 minutes and affected 88% of traffic.

## Timeline

- **2026-01-17 09:12** — Alerts triggered on Elasticsearch metrics
- **2026-01-17 09:18** — Jae-won Kim acknowledged the alert
- **2026-01-17 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2026-01-17 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-17 10:70** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 14,123 requests failed
- 104 users affected
- Downstream services impacted: webhook-handler, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Bob Martins, Jae-won Kim

## Lessons Learned

We need better load testing to catch these issues before production.
