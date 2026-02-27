# Incident Report — media-uploader — 2025-10-25

**Date:** 2025-10-25
**Severity:** P3
**Duration:** ~55 minutes
**Service:** media-uploader
**Responders:** Isabelle Dupont, Elena Rossi

## Summary

Media-uploader experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
55 minutes and affected 60% of traffic.

## Timeline

- **2025-10-25 09:12** — Alerts triggered on Axum metrics
- **2025-10-25 09:18** — Isabelle Dupont acknowledged the alert
- **2025-10-25 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2025-10-25 09:41** — Mitigation applied (rolled back last deployment)
- **2025-10-25 09:67** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 14,443 requests failed
- 17 users affected
- Downstream services impacted: event-bus, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with Isabelle Dupont, Elena Rossi

## Lessons Learned

We need better canary deployments to catch these issues before production.
