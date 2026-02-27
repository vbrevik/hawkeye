# Incident Report — auth-service — 2025-01-07

**Date:** 2025-01-07
**Severity:** P3
**Duration:** ~49 minutes
**Service:** auth-service
**Responders:** Frank Müller, Henrik Larsen, Isabelle Dupont

## Summary

Auth-service experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
49 minutes and affected 48% of traffic.

## Timeline

- **2025-01-07 09:12** — Alerts triggered on Kubernetes metrics
- **2025-01-07 09:18** — Isabelle Dupont acknowledged the alert
- **2025-01-07 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-01-07 09:41** — Mitigation applied (rolled back last deployment)
- **2025-01-07 09:61** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 11,800 requests failed
- 101 users affected
- Downstream services impacted: auth-service, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Frank Müller, Henrik Larsen

## Lessons Learned

We need better load testing to catch these issues before production.
