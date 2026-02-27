# Incident Report — media-uploader — 2024-07-20

**Date:** 2024-07-20
**Severity:** P1
**Duration:** ~235 minutes
**Service:** media-uploader
**Responders:** Jae-won Kim, Gina Torres, Elena Rossi, Quinn Murphy

## Summary

Media-uploader experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
235 minutes and affected 12% of traffic.

## Timeline

- **2024-07-20 09:12** — Alerts triggered on Prometheus metrics
- **2024-07-20 09:18** — Quinn Murphy acknowledged the alert
- **2024-07-20 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2024-07-20 09:41** — Mitigation applied (rolled back last deployment)
- **2024-07-20 12:67** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 44,714 requests failed
- 72 users affected
- Downstream services impacted: report-generator, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Prometheus
- [ ] Schedule blameless post-mortem with Jae-won Kim, Gina Torres

## Lessons Learned

We need better integration tests to catch these issues before production.
