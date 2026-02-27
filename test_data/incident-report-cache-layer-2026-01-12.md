# Incident Report — cache-layer — 2026-01-12

**Date:** 2026-01-12
**Severity:** P2
**Duration:** ~223 minutes
**Service:** cache-layer
**Responders:** David Park, Mohamed Al-Rashid, Oscar Lindberg

## Summary

Cache-layer experienced an outage due to memory leak in the worker pool. The incident lasted approximately
223 minutes and affected 53% of traffic.

## Timeline

- **2026-01-12 09:12** — Alerts triggered on Kubernetes metrics
- **2026-01-12 09:18** — Mohamed Al-Rashid acknowledged the alert
- **2026-01-12 09:25** — Root cause identified: memory leak in the worker pool
- **2026-01-12 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-12 12:55** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 2,458 requests failed
- 17 users affected
- Downstream services impacted: notification-service, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with David Park, Mohamed Al-Rashid

## Lessons Learned

We need better load testing to catch these issues before production.
