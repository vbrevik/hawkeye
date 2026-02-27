# Incident Report — auth-service — 2026-02-26

**Date:** 2026-02-26
**Severity:** P3
**Duration:** ~144 minutes
**Service:** auth-service
**Responders:** Henrik Larsen, Bob Martins

## Summary

Auth-service experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
144 minutes and affected 14% of traffic.

## Timeline

- **2026-02-26 09:12** — Alerts triggered on Elasticsearch metrics
- **2026-02-26 09:18** — Henrik Larsen acknowledged the alert
- **2026-02-26 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2026-02-26 09:41** — Mitigation applied (rolled back last deployment)
- **2026-02-26 11:36** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 37,553 requests failed
- 331 users affected
- Downstream services impacted: data-warehouse, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Henrik Larsen, Bob Martins

## Lessons Learned

We need better alerting coverage to catch these issues before production.
