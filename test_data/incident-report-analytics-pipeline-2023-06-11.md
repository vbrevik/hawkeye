# Incident Report — analytics-pipeline — 2023-06-11

**Date:** 2023-06-11
**Severity:** P3
**Duration:** ~70 minutes
**Service:** analytics-pipeline
**Responders:** Mohamed Al-Rashid, Alice Chen, Frank Müller

## Summary

Analytics-pipeline experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
70 minutes and affected 88% of traffic.

## Timeline

- **2023-06-11 09:12** — Alerts triggered on DynamoDB metrics
- **2023-06-11 09:18** — Frank Müller acknowledged the alert
- **2023-06-11 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2023-06-11 09:41** — Mitigation applied (rolled back last deployment)
- **2023-06-11 10:22** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 28,505 requests failed
- 495 users affected
- Downstream services impacted: api-gateway, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Mohamed Al-Rashid, Alice Chen

## Lessons Learned

We need better canary deployments to catch these issues before production.
