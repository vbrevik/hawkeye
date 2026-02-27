# Incident Report — analytics-pipeline — 2025-02-22

**Date:** 2025-02-22
**Severity:** P2
**Duration:** ~224 minutes
**Service:** analytics-pipeline
**Responders:** Tomas Novak, Quinn Murphy

## Summary

Analytics-pipeline experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
224 minutes and affected 40% of traffic.

## Timeline

- **2025-02-22 09:12** — Alerts triggered on ArgoCD metrics
- **2025-02-22 09:18** — Tomas Novak acknowledged the alert
- **2025-02-22 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-02-22 09:41** — Mitigation applied (rolled back last deployment)
- **2025-02-22 12:56** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 16,034 requests failed
- 83 users affected
- Downstream services impacted: analytics-pipeline, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for ArgoCD
- [ ] Schedule blameless post-mortem with Tomas Novak, Quinn Murphy

## Lessons Learned

We need better integration tests to catch these issues before production.
