# Incident Report — analytics-pipeline — 2025-03-15

**Date:** 2025-03-15
**Severity:** P3
**Duration:** ~123 minutes
**Service:** analytics-pipeline
**Responders:** Ravi Sharma, Frank Müller, Tomas Novak, Alice Chen

## Summary

Analytics-pipeline experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
123 minutes and affected 95% of traffic.

## Timeline

- **2025-03-15 09:12** — Alerts triggered on Kafka metrics
- **2025-03-15 09:18** — Frank Müller acknowledged the alert
- **2025-03-15 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-03-15 09:41** — Mitigation applied (rolled back last deployment)
- **2025-03-15 11:15** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 19,192 requests failed
- 426 users affected
- Downstream services impacted: report-generator, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kafka
- [ ] Schedule blameless post-mortem with Ravi Sharma, Frank Müller

## Lessons Learned

We need better integration tests to catch these issues before production.
