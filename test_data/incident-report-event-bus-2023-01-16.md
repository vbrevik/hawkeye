# Incident Report — event-bus — 2023-01-16

**Date:** 2023-01-16
**Severity:** P3
**Duration:** ~63 minutes
**Service:** event-bus
**Responders:** Mohamed Al-Rashid, Ravi Sharma

## Summary

Event-bus experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
63 minutes and affected 69% of traffic.

## Timeline

- **2023-01-16 09:12** — Alerts triggered on FastAPI metrics
- **2023-01-16 09:18** — Ravi Sharma acknowledged the alert
- **2023-01-16 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2023-01-16 09:41** — Mitigation applied (rolled back last deployment)
- **2023-01-16 10:15** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 26,122 requests failed
- 442 users affected
- Downstream services impacted: report-generator, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Mohamed Al-Rashid, Ravi Sharma

## Lessons Learned

We need better integration tests to catch these issues before production.
