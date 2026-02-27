# Incident Report — notification-service — 2023-07-05

**Date:** 2023-07-05
**Severity:** P2
**Duration:** ~120 minutes
**Service:** notification-service
**Responders:** Laura Bianchi, Clara Johansson

## Summary

Notification-service experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
120 minutes and affected 17% of traffic.

## Timeline

- **2023-07-05 09:12** — Alerts triggered on Terraform metrics
- **2023-07-05 09:18** — Clara Johansson acknowledged the alert
- **2023-07-05 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2023-07-05 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-05 11:12** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 39,825 requests failed
- 32 users affected
- Downstream services impacted: cache-layer, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Terraform
- [ ] Schedule blameless post-mortem with Laura Bianchi, Clara Johansson

## Lessons Learned

We need better load testing to catch these issues before production.
