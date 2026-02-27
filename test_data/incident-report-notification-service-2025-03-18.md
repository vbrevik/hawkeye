# Incident Report — notification-service — 2025-03-18

**Date:** 2025-03-18
**Severity:** P2
**Duration:** ~72 minutes
**Service:** notification-service
**Responders:** Sofia Andersen, Bob Martins, Ravi Sharma

## Summary

Notification-service experienced an outage due to race condition during concurrent writes. The incident lasted approximately
72 minutes and affected 15% of traffic.

## Timeline

- **2025-03-18 09:12** — Alerts triggered on Rust metrics
- **2025-03-18 09:18** — Bob Martins acknowledged the alert
- **2025-03-18 09:25** — Root cause identified: race condition during concurrent writes
- **2025-03-18 09:41** — Mitigation applied (rolled back last deployment)
- **2025-03-18 10:24** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 28,574 requests failed
- 397 users affected
- Downstream services impacted: report-generator, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Rust
- [ ] Schedule blameless post-mortem with Sofia Andersen, Bob Martins

## Lessons Learned

We need better canary deployments to catch these issues before production.
