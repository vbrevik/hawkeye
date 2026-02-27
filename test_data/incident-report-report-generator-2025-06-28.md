# Incident Report — report-generator — 2025-06-28

**Date:** 2025-06-28
**Severity:** P1
**Duration:** ~218 minutes
**Service:** report-generator
**Responders:** Clara Johansson, Alice Chen

## Summary

Report-generator experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
218 minutes and affected 71% of traffic.

## Timeline

- **2025-06-28 09:12** — Alerts triggered on DynamoDB metrics
- **2025-06-28 09:18** — Alice Chen acknowledged the alert
- **2025-06-28 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2025-06-28 09:41** — Mitigation applied (rolled back last deployment)
- **2025-06-28 12:50** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 38,025 requests failed
- 176 users affected
- Downstream services impacted: media-uploader, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Clara Johansson, Alice Chen

## Lessons Learned

We need better load testing to catch these issues before production.
