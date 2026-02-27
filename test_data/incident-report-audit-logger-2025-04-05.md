# Incident Report — audit-logger — 2025-04-05

**Date:** 2025-04-05
**Severity:** P3
**Duration:** ~60 minutes
**Service:** audit-logger
**Responders:** Gina Torres, Clara Johansson, Elena Rossi, David Park

## Summary

Audit-logger experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
60 minutes and affected 40% of traffic.

## Timeline

- **2025-04-05 09:12** — Alerts triggered on Terraform metrics
- **2025-04-05 09:18** — Clara Johansson acknowledged the alert
- **2025-04-05 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-04-05 09:41** — Mitigation applied (rolled back last deployment)
- **2025-04-05 10:12** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 37,040 requests failed
- 53 users affected
- Downstream services impacted: notification-service, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for audit-logger

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Terraform
- [ ] Schedule blameless post-mortem with Gina Torres, Clara Johansson

## Lessons Learned

We need better alerting coverage to catch these issues before production.
