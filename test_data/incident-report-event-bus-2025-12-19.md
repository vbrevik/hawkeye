# Incident Report — event-bus — 2025-12-19

**Date:** 2025-12-19
**Severity:** P1
**Duration:** ~207 minutes
**Service:** event-bus
**Responders:** Oscar Lindberg, Mohamed Al-Rashid

## Summary

Event-bus experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
207 minutes and affected 40% of traffic.

## Timeline

- **2025-12-19 09:12** — Alerts triggered on GraphQL metrics
- **2025-12-19 09:18** — Oscar Lindberg acknowledged the alert
- **2025-12-19 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2025-12-19 09:41** — Mitigation applied (rolled back last deployment)
- **2025-12-19 12:39** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 3,968 requests failed
- 243 users affected
- Downstream services impacted: payment-processor, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for GraphQL
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Mohamed Al-Rashid

## Lessons Learned

We need better staging parity to catch these issues before production.
