# Incident Report — scheduler — 2024-02-17

**Date:** 2024-02-17
**Severity:** P3
**Duration:** ~185 minutes
**Service:** scheduler
**Responders:** Bob Martins, Alice Chen, Elena Rossi

## Summary

Scheduler experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
185 minutes and affected 23% of traffic.

## Timeline

- **2024-02-17 09:12** — Alerts triggered on Rust metrics
- **2024-02-17 09:18** — Bob Martins acknowledged the alert
- **2024-02-17 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2024-02-17 09:41** — Mitigation applied (rolled back last deployment)
- **2024-02-17 12:17** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 24,013 requests failed
- 492 users affected
- Downstream services impacted: api-gateway, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Rust
- [ ] Schedule blameless post-mortem with Bob Martins, Alice Chen

## Lessons Learned

We need better integration tests to catch these issues before production.
