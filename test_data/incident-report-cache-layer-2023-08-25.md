# Incident Report — cache-layer — 2023-08-25

**Date:** 2023-08-25
**Severity:** P2
**Duration:** ~149 minutes
**Service:** cache-layer
**Responders:** Elena Rossi, Oscar Lindberg, Kofi Mensah

## Summary

Cache-layer experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
149 minutes and affected 53% of traffic.

## Timeline

- **2023-08-25 09:12** — Alerts triggered on RabbitMQ metrics
- **2023-08-25 09:18** — Kofi Mensah acknowledged the alert
- **2023-08-25 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2023-08-25 09:41** — Mitigation applied (rolled back last deployment)
- **2023-08-25 11:41** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 2,318 requests failed
- 166 users affected
- Downstream services impacted: report-generator, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for RabbitMQ
- [ ] Schedule blameless post-mortem with Elena Rossi, Oscar Lindberg

## Lessons Learned

We need better load testing to catch these issues before production.
