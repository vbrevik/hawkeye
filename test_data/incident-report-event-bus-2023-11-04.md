# Incident Report — event-bus — 2023-11-04

**Date:** 2023-11-04
**Severity:** P2
**Duration:** ~29 minutes
**Service:** event-bus
**Responders:** Gina Torres, Tomas Novak, David Park, Oscar Lindberg

## Summary

Event-bus experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
29 minutes and affected 12% of traffic.

## Timeline

- **2023-11-04 09:12** — Alerts triggered on Redis metrics
- **2023-11-04 09:18** — Oscar Lindberg acknowledged the alert
- **2023-11-04 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2023-11-04 09:41** — Mitigation applied (rolled back last deployment)
- **2023-11-04 09:41** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 10,353 requests failed
- 35 users affected
- Downstream services impacted: analytics-pipeline, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Redis
- [ ] Schedule blameless post-mortem with Gina Torres, Tomas Novak

## Lessons Learned

We need better alerting coverage to catch these issues before production.
