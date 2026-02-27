# Incident Report — webhook-handler — 2024-09-06

**Date:** 2024-09-06
**Severity:** P1
**Duration:** ~207 minutes
**Service:** webhook-handler
**Responders:** Quinn Murphy, Henrik Larsen, David Park

## Summary

Webhook-handler experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
207 minutes and affected 22% of traffic.

## Timeline

- **2024-09-06 09:12** — Alerts triggered on Go metrics
- **2024-09-06 09:18** — Quinn Murphy acknowledged the alert
- **2024-09-06 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2024-09-06 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-06 12:39** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 1,822 requests failed
- 185 users affected
- Downstream services impacted: cache-layer, search-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Quinn Murphy, Henrik Larsen

## Lessons Learned

We need better load testing to catch these issues before production.
