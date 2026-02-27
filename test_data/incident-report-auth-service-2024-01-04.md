# Incident Report — auth-service — 2024-01-04

**Date:** 2024-01-04
**Severity:** P2
**Duration:** ~140 minutes
**Service:** auth-service
**Responders:** Isabelle Dupont, Bob Martins

## Summary

Auth-service experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
140 minutes and affected 50% of traffic.

## Timeline

- **2024-01-04 09:12** — Alerts triggered on Prometheus metrics
- **2024-01-04 09:18** — Isabelle Dupont acknowledged the alert
- **2024-01-04 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2024-01-04 09:41** — Mitigation applied (rolled back last deployment)
- **2024-01-04 11:32** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,662 requests failed
- 127 users affected
- Downstream services impacted: media-uploader, payment-processor

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Prometheus
- [ ] Schedule blameless post-mortem with Isabelle Dupont, Bob Martins

## Lessons Learned

We need better load testing to catch these issues before production.
