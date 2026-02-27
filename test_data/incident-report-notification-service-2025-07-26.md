# Incident Report — notification-service — 2025-07-26

**Date:** 2025-07-26
**Severity:** P3
**Duration:** ~50 minutes
**Service:** notification-service
**Responders:** Priya Patel, Kofi Mensah

## Summary

Notification-service experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
50 minutes and affected 45% of traffic.

## Timeline

- **2025-07-26 09:12** — Alerts triggered on Prometheus metrics
- **2025-07-26 09:18** — Priya Patel acknowledged the alert
- **2025-07-26 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2025-07-26 09:41** — Mitigation applied (rolled back last deployment)
- **2025-07-26 09:62** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 29,725 requests failed
- 141 users affected
- Downstream services impacted: payment-processor, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Prometheus
- [ ] Schedule blameless post-mortem with Priya Patel, Kofi Mensah

## Lessons Learned

We need better canary deployments to catch these issues before production.
