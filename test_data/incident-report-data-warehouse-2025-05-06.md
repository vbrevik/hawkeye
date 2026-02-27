# Incident Report — data-warehouse — 2025-05-06

**Date:** 2025-05-06
**Severity:** P3
**Duration:** ~151 minutes
**Service:** data-warehouse
**Responders:** Henrik Larsen, Elena Rossi, Quinn Murphy, Clara Johansson

## Summary

Data-warehouse experienced an outage due to memory leak in the worker pool. The incident lasted approximately
151 minutes and affected 57% of traffic.

## Timeline

- **2025-05-06 09:12** — Alerts triggered on gRPC metrics
- **2025-05-06 09:18** — Henrik Larsen acknowledged the alert
- **2025-05-06 09:25** — Root cause identified: memory leak in the worker pool
- **2025-05-06 09:41** — Mitigation applied (rolled back last deployment)
- **2025-05-06 11:43** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 40,064 requests failed
- 128 users affected
- Downstream services impacted: notification-service, search-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for gRPC
- [ ] Schedule blameless post-mortem with Henrik Larsen, Elena Rossi

## Lessons Learned

We need better integration tests to catch these issues before production.
