# Incident Report — cache-layer — 2025-08-14

**Date:** 2025-08-14
**Severity:** P3
**Duration:** ~107 minutes
**Service:** cache-layer
**Responders:** Tomas Novak, Oscar Lindberg, Nadia Kovač

## Summary

Cache-layer experienced an outage due to memory leak in the worker pool. The incident lasted approximately
107 minutes and affected 65% of traffic.

## Timeline

- **2025-08-14 09:12** — Alerts triggered on Go metrics
- **2025-08-14 09:18** — Tomas Novak acknowledged the alert
- **2025-08-14 09:25** — Root cause identified: memory leak in the worker pool
- **2025-08-14 09:41** — Mitigation applied (rolled back last deployment)
- **2025-08-14 10:59** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 47,506 requests failed
- 192 users affected
- Downstream services impacted: search-service, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Tomas Novak, Oscar Lindberg

## Lessons Learned

We need better integration tests to catch these issues before production.
