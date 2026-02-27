# Incident Report — report-generator — 2023-11-10

**Date:** 2023-11-10
**Severity:** P1
**Duration:** ~118 minutes
**Service:** report-generator
**Responders:** Nadia Kovač, David Park

## Summary

Report-generator experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
118 minutes and affected 86% of traffic.

## Timeline

- **2023-11-10 09:12** — Alerts triggered on FastAPI metrics
- **2023-11-10 09:18** — Nadia Kovač acknowledged the alert
- **2023-11-10 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2023-11-10 09:41** — Mitigation applied (rolled back last deployment)
- **2023-11-10 10:70** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,488 requests failed
- 440 users affected
- Downstream services impacted: cache-layer, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Nadia Kovač, David Park

## Lessons Learned

We need better canary deployments to catch these issues before production.
