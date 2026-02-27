# Incident Report — search-service — 2024-12-05

**Date:** 2024-12-05
**Severity:** P1
**Duration:** ~146 minutes
**Service:** search-service
**Responders:** Oscar Lindberg, Ravi Sharma, David Park, Mohamed Al-Rashid

## Summary

Search-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
146 minutes and affected 42% of traffic.

## Timeline

- **2024-12-05 09:12** — Alerts triggered on Terraform metrics
- **2024-12-05 09:18** — David Park acknowledged the alert
- **2024-12-05 09:25** — Root cause identified: memory leak in the worker pool
- **2024-12-05 09:41** — Mitigation applied (rolled back last deployment)
- **2024-12-05 11:38** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 12,701 requests failed
- 361 users affected
- Downstream services impacted: api-gateway, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for search-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Terraform
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Ravi Sharma

## Lessons Learned

We need better canary deployments to catch these issues before production.
