# Incident Report — event-bus — 2025-10-21

**Date:** 2025-10-21
**Severity:** P3
**Duration:** ~37 minutes
**Service:** event-bus
**Responders:** Sofia Andersen, Tomas Novak, Frank Müller, Clara Johansson

## Summary

Event-bus experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
37 minutes and affected 13% of traffic.

## Timeline

- **2025-10-21 09:12** — Alerts triggered on Go metrics
- **2025-10-21 09:18** — Tomas Novak acknowledged the alert
- **2025-10-21 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-10-21 09:41** — Mitigation applied (rolled back last deployment)
- **2025-10-21 09:49** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 37,449 requests failed
- 439 users affected
- Downstream services impacted: payment-processor, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Sofia Andersen, Tomas Novak

## Lessons Learned

We need better load testing to catch these issues before production.
