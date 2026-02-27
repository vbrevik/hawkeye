# Incident Report — payment-processor — 2024-07-13

**Date:** 2024-07-13
**Severity:** P3
**Duration:** ~178 minutes
**Service:** payment-processor
**Responders:** Nadia Kovač, David Park, Jae-won Kim

## Summary

Payment-processor experienced an outage due to race condition during concurrent writes. The incident lasted approximately
178 minutes and affected 80% of traffic.

## Timeline

- **2024-07-13 09:12** — Alerts triggered on Axum metrics
- **2024-07-13 09:18** — David Park acknowledged the alert
- **2024-07-13 09:25** — Root cause identified: race condition during concurrent writes
- **2024-07-13 09:41** — Mitigation applied (rolled back last deployment)
- **2024-07-13 11:70** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 43,821 requests failed
- 158 users affected
- Downstream services impacted: auth-service, payment-processor

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for payment-processor

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with Nadia Kovač, David Park

## Lessons Learned

We need better integration tests to catch these issues before production.
