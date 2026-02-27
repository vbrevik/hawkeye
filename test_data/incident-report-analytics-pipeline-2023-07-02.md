# Incident Report — analytics-pipeline — 2023-07-02

**Date:** 2023-07-02
**Severity:** P2
**Duration:** ~216 minutes
**Service:** analytics-pipeline
**Responders:** Quinn Murphy, Nadia Kovač

## Summary

Analytics-pipeline experienced an outage due to race condition during concurrent writes. The incident lasted approximately
216 minutes and affected 91% of traffic.

## Timeline

- **2023-07-02 09:12** — Alerts triggered on Go metrics
- **2023-07-02 09:18** — Nadia Kovač acknowledged the alert
- **2023-07-02 09:25** — Root cause identified: race condition during concurrent writes
- **2023-07-02 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-02 12:48** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 14,811 requests failed
- 366 users affected
- Downstream services impacted: payment-processor, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Quinn Murphy, Nadia Kovač

## Lessons Learned

We need better load testing to catch these issues before production.
