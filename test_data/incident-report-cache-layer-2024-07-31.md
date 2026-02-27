# Incident Report — cache-layer — 2024-07-31

**Date:** 2024-07-31
**Severity:** P2
**Duration:** ~51 minutes
**Service:** cache-layer
**Responders:** Frank Müller, Kofi Mensah

## Summary

Cache-layer experienced an outage due to race condition during concurrent writes. The incident lasted approximately
51 minutes and affected 49% of traffic.

## Timeline

- **2024-07-31 09:12** — Alerts triggered on Terraform metrics
- **2024-07-31 09:18** — Kofi Mensah acknowledged the alert
- **2024-07-31 09:25** — Root cause identified: race condition during concurrent writes
- **2024-07-31 09:41** — Mitigation applied (rolled back last deployment)
- **2024-07-31 09:63** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 38,917 requests failed
- 66 users affected
- Downstream services impacted: user-service, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Terraform
- [ ] Schedule blameless post-mortem with Frank Müller, Kofi Mensah

## Lessons Learned

We need better alerting coverage to catch these issues before production.
