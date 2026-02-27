# Incident Report — api-gateway — 2024-09-15

**Date:** 2024-09-15
**Severity:** P3
**Duration:** ~212 minutes
**Service:** api-gateway
**Responders:** Ravi Sharma, Elena Rossi, Kofi Mensah, Laura Bianchi

## Summary

Api-gateway experienced an outage due to memory leak in the worker pool. The incident lasted approximately
212 minutes and affected 51% of traffic.

## Timeline

- **2024-09-15 09:12** — Alerts triggered on Kafka metrics
- **2024-09-15 09:18** — Kofi Mensah acknowledged the alert
- **2024-09-15 09:25** — Root cause identified: memory leak in the worker pool
- **2024-09-15 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-15 12:44** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 42,714 requests failed
- 142 users affected
- Downstream services impacted: payment-processor, search-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kafka
- [ ] Schedule blameless post-mortem with Ravi Sharma, Elena Rossi

## Lessons Learned

We need better integration tests to catch these issues before production.
