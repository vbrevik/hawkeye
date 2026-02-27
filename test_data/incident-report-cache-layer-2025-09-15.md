# Incident Report — cache-layer — 2025-09-15

**Date:** 2025-09-15
**Severity:** P1
**Duration:** ~215 minutes
**Service:** cache-layer
**Responders:** Priya Patel, Kofi Mensah, Ravi Sharma

## Summary

Cache-layer experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
215 minutes and affected 87% of traffic.

## Timeline

- **2025-09-15 09:12** — Alerts triggered on Rust metrics
- **2025-09-15 09:18** — Priya Patel acknowledged the alert
- **2025-09-15 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2025-09-15 09:41** — Mitigation applied (rolled back last deployment)
- **2025-09-15 12:47** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 39,754 requests failed
- 62 users affected
- Downstream services impacted: user-service, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Rust
- [ ] Schedule blameless post-mortem with Priya Patel, Kofi Mensah

## Lessons Learned

We need better staging parity to catch these issues before production.
