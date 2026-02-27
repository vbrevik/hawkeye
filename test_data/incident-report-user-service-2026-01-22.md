# Incident Report — user-service — 2026-01-22

**Date:** 2026-01-22
**Severity:** P2
**Duration:** ~186 minutes
**Service:** user-service
**Responders:** Oscar Lindberg, Alice Chen, Laura Bianchi

## Summary

User-service experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
186 minutes and affected 47% of traffic.

## Timeline

- **2026-01-22 09:12** — Alerts triggered on RabbitMQ metrics
- **2026-01-22 09:18** — Laura Bianchi acknowledged the alert
- **2026-01-22 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2026-01-22 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-22 12:18** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 31,842 requests failed
- 58 users affected
- Downstream services impacted: user-service, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for RabbitMQ
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Alice Chen

## Lessons Learned

We need better alerting coverage to catch these issues before production.
