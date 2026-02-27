# Incident Report — analytics-pipeline — 2025-10-15

**Date:** 2025-10-15
**Severity:** P2
**Duration:** ~170 minutes
**Service:** analytics-pipeline
**Responders:** Oscar Lindberg, Henrik Larsen

## Summary

Analytics-pipeline experienced an outage due to memory leak in the worker pool. The incident lasted approximately
170 minutes and affected 14% of traffic.

## Timeline

- **2025-10-15 09:12** — Alerts triggered on Terraform metrics
- **2025-10-15 09:18** — Henrik Larsen acknowledged the alert
- **2025-10-15 09:25** — Root cause identified: memory leak in the worker pool
- **2025-10-15 09:41** — Mitigation applied (rolled back last deployment)
- **2025-10-15 11:62** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 19,401 requests failed
- 432 users affected
- Downstream services impacted: cache-layer, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Terraform
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Henrik Larsen

## Lessons Learned

We need better alerting coverage to catch these issues before production.
