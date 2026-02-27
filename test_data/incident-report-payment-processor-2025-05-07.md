# Incident Report — payment-processor — 2025-05-07

**Date:** 2025-05-07
**Severity:** P3
**Duration:** ~40 minutes
**Service:** payment-processor
**Responders:** Nadia Kovač, Sofia Andersen

## Summary

Payment-processor experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
40 minutes and affected 82% of traffic.

## Timeline

- **2025-05-07 09:12** — Alerts triggered on Helm metrics
- **2025-05-07 09:18** — Sofia Andersen acknowledged the alert
- **2025-05-07 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-05-07 09:41** — Mitigation applied (rolled back last deployment)
- **2025-05-07 09:52** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 22,485 requests failed
- 161 users affected
- Downstream services impacted: api-gateway, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for payment-processor

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Helm
- [ ] Schedule blameless post-mortem with Nadia Kovač, Sofia Andersen

## Lessons Learned

We need better integration tests to catch these issues before production.
