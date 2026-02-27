# Incident Report — scheduler — 2025-07-11

**Date:** 2025-07-11
**Severity:** P2
**Duration:** ~100 minutes
**Service:** scheduler
**Responders:** Tomas Novak, David Park, Elena Rossi, Sofia Andersen

## Summary

Scheduler experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
100 minutes and affected 48% of traffic.

## Timeline

- **2025-07-11 09:12** — Alerts triggered on ArgoCD metrics
- **2025-07-11 09:18** — Elena Rossi acknowledged the alert
- **2025-07-11 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2025-07-11 09:41** — Mitigation applied (rolled back last deployment)
- **2025-07-11 10:52** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 13,145 requests failed
- 309 users affected
- Downstream services impacted: cache-layer, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for ArgoCD
- [ ] Schedule blameless post-mortem with Tomas Novak, David Park

## Lessons Learned

We need better alerting coverage to catch these issues before production.
