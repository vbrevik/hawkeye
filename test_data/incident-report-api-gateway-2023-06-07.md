# Incident Report — api-gateway — 2023-06-07

**Date:** 2023-06-07
**Severity:** P3
**Duration:** ~165 minutes
**Service:** api-gateway
**Responders:** Elena Rossi, Priya Patel, Tomas Novak, Gina Torres

## Summary

Api-gateway experienced an outage due to race condition during concurrent writes. The incident lasted approximately
165 minutes and affected 7% of traffic.

## Timeline

- **2023-06-07 09:12** — Alerts triggered on Go metrics
- **2023-06-07 09:18** — Tomas Novak acknowledged the alert
- **2023-06-07 09:25** — Root cause identified: race condition during concurrent writes
- **2023-06-07 09:41** — Mitigation applied (rolled back last deployment)
- **2023-06-07 11:57** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 44,821 requests failed
- 278 users affected
- Downstream services impacted: audit-logger, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Elena Rossi, Priya Patel

## Lessons Learned

We need better integration tests to catch these issues before production.
