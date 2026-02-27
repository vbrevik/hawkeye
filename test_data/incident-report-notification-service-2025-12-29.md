# Incident Report — notification-service — 2025-12-29

**Date:** 2025-12-29
**Severity:** P1
**Duration:** ~151 minutes
**Service:** notification-service
**Responders:** Jae-won Kim, Elena Rossi, Priya Patel, David Park

## Summary

Notification-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
151 minutes and affected 41% of traffic.

## Timeline

- **2025-12-29 09:12** — Alerts triggered on Terraform metrics
- **2025-12-29 09:18** — David Park acknowledged the alert
- **2025-12-29 09:25** — Root cause identified: memory leak in the worker pool
- **2025-12-29 09:41** — Mitigation applied (rolled back last deployment)
- **2025-12-29 11:43** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 28,966 requests failed
- 175 users affected
- Downstream services impacted: notification-service, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Terraform
- [ ] Schedule blameless post-mortem with Jae-won Kim, Elena Rossi

## Lessons Learned

We need better load testing to catch these issues before production.
