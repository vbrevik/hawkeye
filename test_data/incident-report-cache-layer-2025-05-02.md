# Incident Report — cache-layer — 2025-05-02

**Date:** 2025-05-02
**Severity:** P3
**Duration:** ~123 minutes
**Service:** cache-layer
**Responders:** Jae-won Kim, Sofia Andersen

## Summary

Cache-layer experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
123 minutes and affected 74% of traffic.

## Timeline

- **2025-05-02 09:12** — Alerts triggered on TypeScript metrics
- **2025-05-02 09:18** — Jae-won Kim acknowledged the alert
- **2025-05-02 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2025-05-02 09:41** — Mitigation applied (rolled back last deployment)
- **2025-05-02 11:15** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 28,732 requests failed
- 50 users affected
- Downstream services impacted: notification-service, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for TypeScript
- [ ] Schedule blameless post-mortem with Jae-won Kim, Sofia Andersen

## Lessons Learned

We need better integration tests to catch these issues before production.
