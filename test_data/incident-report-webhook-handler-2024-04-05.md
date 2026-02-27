# Incident Report — webhook-handler — 2024-04-05

**Date:** 2024-04-05
**Severity:** P3
**Duration:** ~74 minutes
**Service:** webhook-handler
**Responders:** Elena Rossi, Priya Patel, Jae-won Kim, Quinn Murphy

## Summary

Webhook-handler experienced an outage due to race condition during concurrent writes. The incident lasted approximately
74 minutes and affected 58% of traffic.

## Timeline

- **2024-04-05 09:12** — Alerts triggered on Kafka metrics
- **2024-04-05 09:18** — Quinn Murphy acknowledged the alert
- **2024-04-05 09:25** — Root cause identified: race condition during concurrent writes
- **2024-04-05 09:41** — Mitigation applied (rolled back last deployment)
- **2024-04-05 10:26** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 16,073 requests failed
- 234 users affected
- Downstream services impacted: cache-layer, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kafka
- [ ] Schedule blameless post-mortem with Elena Rossi, Priya Patel

## Lessons Learned

We need better integration tests to catch these issues before production.
