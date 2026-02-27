# Incident Report — search-service — 2025-04-15

**Date:** 2025-04-15
**Severity:** P1
**Duration:** ~218 minutes
**Service:** search-service
**Responders:** Quinn Murphy, Priya Patel

## Summary

Search-service experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
218 minutes and affected 31% of traffic.

## Timeline

- **2025-04-15 09:12** — Alerts triggered on ArgoCD metrics
- **2025-04-15 09:18** — Priya Patel acknowledged the alert
- **2025-04-15 09:25** — Root cause identified: retry storm after upstream timeout
- **2025-04-15 09:41** — Mitigation applied (rolled back last deployment)
- **2025-04-15 12:50** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 23,175 requests failed
- 183 users affected
- Downstream services impacted: search-service, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for search-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for ArgoCD
- [ ] Schedule blameless post-mortem with Quinn Murphy, Priya Patel

## Lessons Learned

We need better canary deployments to catch these issues before production.
