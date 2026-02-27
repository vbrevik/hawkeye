# Incident Report — search-service — 2024-08-30

**Date:** 2024-08-30
**Severity:** P2
**Duration:** ~193 minutes
**Service:** search-service
**Responders:** Nadia Kovač, Isabelle Dupont

## Summary

Search-service experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
193 minutes and affected 35% of traffic.

## Timeline

- **2024-08-30 09:12** — Alerts triggered on gRPC metrics
- **2024-08-30 09:18** — Nadia Kovač acknowledged the alert
- **2024-08-30 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2024-08-30 09:41** — Mitigation applied (rolled back last deployment)
- **2024-08-30 12:25** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 18,545 requests failed
- 387 users affected
- Downstream services impacted: notification-service, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for search-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for gRPC
- [ ] Schedule blameless post-mortem with Nadia Kovač, Isabelle Dupont

## Lessons Learned

We need better staging parity to catch these issues before production.
