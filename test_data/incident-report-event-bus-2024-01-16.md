# Incident Report — event-bus — 2024-01-16

**Date:** 2024-01-16
**Severity:** P1
**Duration:** ~85 minutes
**Service:** event-bus
**Responders:** Frank Müller, Priya Patel, Jae-won Kim, Elena Rossi

## Summary

Event-bus experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
85 minutes and affected 62% of traffic.

## Timeline

- **2024-01-16 09:12** — Alerts triggered on DynamoDB metrics
- **2024-01-16 09:18** — Frank Müller acknowledged the alert
- **2024-01-16 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2024-01-16 09:41** — Mitigation applied (rolled back last deployment)
- **2024-01-16 10:37** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 728 requests failed
- 249 users affected
- Downstream services impacted: notification-service, payment-processor

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Frank Müller, Priya Patel

## Lessons Learned

We need better integration tests to catch these issues before production.
