# Incident Report — cache-layer — 2023-06-09

**Date:** 2023-06-09
**Severity:** P2
**Duration:** ~232 minutes
**Service:** cache-layer
**Responders:** Mohamed Al-Rashid, Laura Bianchi, Sofia Andersen, Priya Patel

## Summary

Cache-layer experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
232 minutes and affected 82% of traffic.

## Timeline

- **2023-06-09 09:12** — Alerts triggered on Terraform metrics
- **2023-06-09 09:18** — Mohamed Al-Rashid acknowledged the alert
- **2023-06-09 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2023-06-09 09:41** — Mitigation applied (rolled back last deployment)
- **2023-06-09 12:64** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 9,207 requests failed
- 226 users affected
- Downstream services impacted: notification-service, payment-processor

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Terraform
- [ ] Schedule blameless post-mortem with Mohamed Al-Rashid, Laura Bianchi

## Lessons Learned

We need better staging parity to catch these issues before production.
