# Incident Report — auth-service — 2023-07-27

**Date:** 2023-07-27
**Severity:** P2
**Duration:** ~39 minutes
**Service:** auth-service
**Responders:** Laura Bianchi, Oscar Lindberg, Jae-won Kim, David Park

## Summary

Auth-service experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
39 minutes and affected 14% of traffic.

## Timeline

- **2023-07-27 09:12** — Alerts triggered on DynamoDB metrics
- **2023-07-27 09:18** — Jae-won Kim acknowledged the alert
- **2023-07-27 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2023-07-27 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-27 09:51** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 10,046 requests failed
- 448 users affected
- Downstream services impacted: notification-service, search-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Laura Bianchi, Oscar Lindberg

## Lessons Learned

We need better load testing to catch these issues before production.
