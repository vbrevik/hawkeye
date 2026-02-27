# Incident Report — media-uploader — 2024-05-01

**Date:** 2024-05-01
**Severity:** P2
**Duration:** ~142 minutes
**Service:** media-uploader
**Responders:** Oscar Lindberg, Priya Patel, Elena Rossi

## Summary

Media-uploader experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
142 minutes and affected 51% of traffic.

## Timeline

- **2024-05-01 09:12** — Alerts triggered on Nginx metrics
- **2024-05-01 09:18** — Oscar Lindberg acknowledged the alert
- **2024-05-01 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2024-05-01 09:41** — Mitigation applied (rolled back last deployment)
- **2024-05-01 11:34** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 18,899 requests failed
- 327 users affected
- Downstream services impacted: report-generator, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Nginx
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Priya Patel

## Lessons Learned

We need better integration tests to catch these issues before production.
