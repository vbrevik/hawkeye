# Incident Report — api-gateway — 2023-11-30

**Date:** 2023-11-30
**Severity:** P3
**Duration:** ~181 minutes
**Service:** api-gateway
**Responders:** Gina Torres, Ravi Sharma

## Summary

Api-gateway experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
181 minutes and affected 72% of traffic.

## Timeline

- **2023-11-30 09:12** — Alerts triggered on TypeScript metrics
- **2023-11-30 09:18** — Gina Torres acknowledged the alert
- **2023-11-30 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2023-11-30 09:41** — Mitigation applied (rolled back last deployment)
- **2023-11-30 12:13** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 30,278 requests failed
- 470 users affected
- Downstream services impacted: api-gateway, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for TypeScript
- [ ] Schedule blameless post-mortem with Gina Torres, Ravi Sharma

## Lessons Learned

We need better canary deployments to catch these issues before production.
