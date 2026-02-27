# Incident Report — user-service — 2025-09-15

**Date:** 2025-09-15
**Severity:** P3
**Duration:** ~206 minutes
**Service:** user-service
**Responders:** Gina Torres, Alice Chen, Laura Bianchi, David Park

## Summary

User-service experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
206 minutes and affected 23% of traffic.

## Timeline

- **2025-09-15 09:12** — Alerts triggered on Rust metrics
- **2025-09-15 09:18** — David Park acknowledged the alert
- **2025-09-15 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-09-15 09:41** — Mitigation applied (rolled back last deployment)
- **2025-09-15 12:38** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 44,628 requests failed
- 431 users affected
- Downstream services impacted: notification-service, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Rust
- [ ] Schedule blameless post-mortem with Gina Torres, Alice Chen

## Lessons Learned

We need better staging parity to catch these issues before production.
