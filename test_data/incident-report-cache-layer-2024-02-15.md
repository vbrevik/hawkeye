# Incident Report — cache-layer — 2024-02-15

**Date:** 2024-02-15
**Severity:** P3
**Duration:** ~217 minutes
**Service:** cache-layer
**Responders:** Ravi Sharma, Jae-won Kim

## Summary

Cache-layer experienced an outage due to race condition during concurrent writes. The incident lasted approximately
217 minutes and affected 76% of traffic.

## Timeline

- **2024-02-15 09:12** — Alerts triggered on Kubernetes metrics
- **2024-02-15 09:18** — Jae-won Kim acknowledged the alert
- **2024-02-15 09:25** — Root cause identified: race condition during concurrent writes
- **2024-02-15 09:41** — Mitigation applied (rolled back last deployment)
- **2024-02-15 12:49** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 18,221 requests failed
- 116 users affected
- Downstream services impacted: search-service, payment-processor

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Ravi Sharma, Jae-won Kim

## Lessons Learned

We need better integration tests to catch these issues before production.
