# Incident Report — media-uploader — 2023-04-19

**Date:** 2023-04-19
**Severity:** P1
**Duration:** ~219 minutes
**Service:** media-uploader
**Responders:** Jae-won Kim, Isabelle Dupont

## Summary

Media-uploader experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
219 minutes and affected 12% of traffic.

## Timeline

- **2023-04-19 09:12** — Alerts triggered on PostgreSQL metrics
- **2023-04-19 09:18** — Isabelle Dupont acknowledged the alert
- **2023-04-19 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2023-04-19 09:41** — Mitigation applied (rolled back last deployment)
- **2023-04-19 12:51** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 49,487 requests failed
- 278 users affected
- Downstream services impacted: auth-service, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for PostgreSQL
- [ ] Schedule blameless post-mortem with Jae-won Kim, Isabelle Dupont

## Lessons Learned

We need better integration tests to catch these issues before production.
