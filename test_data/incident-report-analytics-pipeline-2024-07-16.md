# Incident Report — analytics-pipeline — 2024-07-16

**Date:** 2024-07-16
**Severity:** P2
**Duration:** ~15 minutes
**Service:** analytics-pipeline
**Responders:** Quinn Murphy, Sofia Andersen

## Summary

Analytics-pipeline experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
15 minutes and affected 14% of traffic.

## Timeline

- **2024-07-16 09:12** — Alerts triggered on GraphQL metrics
- **2024-07-16 09:18** — Sofia Andersen acknowledged the alert
- **2024-07-16 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2024-07-16 09:41** — Mitigation applied (rolled back last deployment)
- **2024-07-16 09:27** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 2,609 requests failed
- 76 users affected
- Downstream services impacted: search-service, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for GraphQL
- [ ] Schedule blameless post-mortem with Quinn Murphy, Sofia Andersen

## Lessons Learned

We need better staging parity to catch these issues before production.
