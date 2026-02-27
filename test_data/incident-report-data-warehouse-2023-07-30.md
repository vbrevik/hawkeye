# Incident Report — data-warehouse — 2023-07-30

**Date:** 2023-07-30
**Severity:** P2
**Duration:** ~79 minutes
**Service:** data-warehouse
**Responders:** Clara Johansson, Bob Martins

## Summary

Data-warehouse experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
79 minutes and affected 63% of traffic.

## Timeline

- **2023-07-30 09:12** — Alerts triggered on Vault metrics
- **2023-07-30 09:18** — Bob Martins acknowledged the alert
- **2023-07-30 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2023-07-30 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-30 10:31** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 45,108 requests failed
- 69 users affected
- Downstream services impacted: analytics-pipeline, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Vault
- [ ] Schedule blameless post-mortem with Clara Johansson, Bob Martins

## Lessons Learned

We need better load testing to catch these issues before production.
