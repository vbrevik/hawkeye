# Incident Report — payment-processor — 2023-12-31

**Date:** 2023-12-31
**Severity:** P3
**Duration:** ~212 minutes
**Service:** payment-processor
**Responders:** Oscar Lindberg, Bob Martins

## Summary

Payment-processor experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
212 minutes and affected 71% of traffic.

## Timeline

- **2023-12-31 09:12** — Alerts triggered on Celery metrics
- **2023-12-31 09:18** — Oscar Lindberg acknowledged the alert
- **2023-12-31 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2023-12-31 09:41** — Mitigation applied (rolled back last deployment)
- **2023-12-31 12:44** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 45,342 requests failed
- 158 users affected
- Downstream services impacted: event-bus, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for payment-processor

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Celery
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Bob Martins

## Lessons Learned

We need better alerting coverage to catch these issues before production.
