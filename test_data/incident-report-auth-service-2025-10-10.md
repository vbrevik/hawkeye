# Incident Report — auth-service — 2025-10-10

**Date:** 2025-10-10
**Severity:** P3
**Duration:** ~226 minutes
**Service:** auth-service
**Responders:** Oscar Lindberg, Priya Patel, David Park

## Summary

Auth-service experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
226 minutes and affected 23% of traffic.

## Timeline

- **2025-10-10 09:12** — Alerts triggered on GraphQL metrics
- **2025-10-10 09:18** — Oscar Lindberg acknowledged the alert
- **2025-10-10 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-10-10 09:41** — Mitigation applied (rolled back last deployment)
- **2025-10-10 12:58** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,232 requests failed
- 105 users affected
- Downstream services impacted: webhook-handler, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for GraphQL
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Priya Patel

## Lessons Learned

We need better canary deployments to catch these issues before production.
