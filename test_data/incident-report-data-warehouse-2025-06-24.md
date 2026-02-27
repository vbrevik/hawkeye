# Incident Report — data-warehouse — 2025-06-24

**Date:** 2025-06-24
**Severity:** P3
**Duration:** ~214 minutes
**Service:** data-warehouse
**Responders:** Priya Patel, Henrik Larsen, Kofi Mensah

## Summary

Data-warehouse experienced an outage due to memory leak in the worker pool. The incident lasted approximately
214 minutes and affected 10% of traffic.

## Timeline

- **2025-06-24 09:12** — Alerts triggered on Elasticsearch metrics
- **2025-06-24 09:18** — Henrik Larsen acknowledged the alert
- **2025-06-24 09:25** — Root cause identified: memory leak in the worker pool
- **2025-06-24 09:41** — Mitigation applied (rolled back last deployment)
- **2025-06-24 12:46** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 26,957 requests failed
- 114 users affected
- Downstream services impacted: search-service, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Priya Patel, Henrik Larsen

## Lessons Learned

We need better alerting coverage to catch these issues before production.
