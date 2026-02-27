# Incident Report — data-warehouse — 2023-08-10

**Date:** 2023-08-10
**Severity:** P2
**Duration:** ~215 minutes
**Service:** data-warehouse
**Responders:** Oscar Lindberg, Laura Bianchi

## Summary

Data-warehouse experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
215 minutes and affected 34% of traffic.

## Timeline

- **2023-08-10 09:12** — Alerts triggered on PostgreSQL metrics
- **2023-08-10 09:18** — Oscar Lindberg acknowledged the alert
- **2023-08-10 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2023-08-10 09:41** — Mitigation applied (rolled back last deployment)
- **2023-08-10 12:47** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 43,355 requests failed
- 99 users affected
- Downstream services impacted: data-warehouse, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for PostgreSQL
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Laura Bianchi

## Lessons Learned

We need better load testing to catch these issues before production.
