# Incident Report — analytics-pipeline — 2023-06-13

**Date:** 2023-06-13
**Severity:** P3
**Duration:** ~118 minutes
**Service:** analytics-pipeline
**Responders:** Tomas Novak, Nadia Kovač, Frank Müller

## Summary

Analytics-pipeline experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
118 minutes and affected 49% of traffic.

## Timeline

- **2023-06-13 09:12** — Alerts triggered on RabbitMQ metrics
- **2023-06-13 09:18** — Nadia Kovač acknowledged the alert
- **2023-06-13 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2023-06-13 09:41** — Mitigation applied (rolled back last deployment)
- **2023-06-13 10:70** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 47,732 requests failed
- 446 users affected
- Downstream services impacted: audit-logger, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for RabbitMQ
- [ ] Schedule blameless post-mortem with Tomas Novak, Nadia Kovač

## Lessons Learned

We need better alerting coverage to catch these issues before production.
