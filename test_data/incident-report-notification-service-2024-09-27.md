# Incident Report — notification-service — 2024-09-27

**Date:** 2024-09-27
**Severity:** P1
**Duration:** ~114 minutes
**Service:** notification-service
**Responders:** Clara Johansson, Elena Rossi

## Summary

Notification-service experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
114 minutes and affected 36% of traffic.

## Timeline

- **2024-09-27 09:12** — Alerts triggered on Go metrics
- **2024-09-27 09:18** — Elena Rossi acknowledged the alert
- **2024-09-27 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2024-09-27 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-27 10:66** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,668 requests failed
- 371 users affected
- Downstream services impacted: cache-layer, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Clara Johansson, Elena Rossi

## Lessons Learned

We need better staging parity to catch these issues before production.
