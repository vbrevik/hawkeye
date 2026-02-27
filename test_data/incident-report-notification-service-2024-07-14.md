# Incident Report — notification-service — 2024-07-14

**Date:** 2024-07-14
**Severity:** P3
**Duration:** ~162 minutes
**Service:** notification-service
**Responders:** Frank Müller, Alice Chen

## Summary

Notification-service experienced an outage due to race condition during concurrent writes. The incident lasted approximately
162 minutes and affected 39% of traffic.

## Timeline

- **2024-07-14 09:12** — Alerts triggered on Axum metrics
- **2024-07-14 09:18** — Frank Müller acknowledged the alert
- **2024-07-14 09:25** — Root cause identified: race condition during concurrent writes
- **2024-07-14 09:41** — Mitigation applied (rolled back last deployment)
- **2024-07-14 11:54** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 15,316 requests failed
- 119 users affected
- Downstream services impacted: media-uploader, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with Frank Müller, Alice Chen

## Lessons Learned

We need better load testing to catch these issues before production.
