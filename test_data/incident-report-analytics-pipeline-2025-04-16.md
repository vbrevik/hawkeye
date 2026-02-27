# Incident Report — analytics-pipeline — 2025-04-16

**Date:** 2025-04-16
**Severity:** P3
**Duration:** ~196 minutes
**Service:** analytics-pipeline
**Responders:** Elena Rossi, Gina Torres, Nadia Kovač, Mohamed Al-Rashid

## Summary

Analytics-pipeline experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
196 minutes and affected 27% of traffic.

## Timeline

- **2025-04-16 09:12** — Alerts triggered on Elasticsearch metrics
- **2025-04-16 09:18** — Nadia Kovač acknowledged the alert
- **2025-04-16 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2025-04-16 09:41** — Mitigation applied (rolled back last deployment)
- **2025-04-16 12:28** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 36,009 requests failed
- 427 users affected
- Downstream services impacted: auth-service, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Elena Rossi, Gina Torres

## Lessons Learned

We need better load testing to catch these issues before production.
