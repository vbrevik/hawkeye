# Incident Report — search-service — 2024-08-20

**Date:** 2024-08-20
**Severity:** P3
**Duration:** ~131 minutes
**Service:** search-service
**Responders:** Oscar Lindberg, Laura Bianchi

## Summary

Search-service experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
131 minutes and affected 30% of traffic.

## Timeline

- **2024-08-20 09:12** — Alerts triggered on SQLite metrics
- **2024-08-20 09:18** — Oscar Lindberg acknowledged the alert
- **2024-08-20 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2024-08-20 09:41** — Mitigation applied (rolled back last deployment)
- **2024-08-20 11:23** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 517 requests failed
- 478 users affected
- Downstream services impacted: data-warehouse, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for search-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for SQLite
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Laura Bianchi

## Lessons Learned

We need better canary deployments to catch these issues before production.
