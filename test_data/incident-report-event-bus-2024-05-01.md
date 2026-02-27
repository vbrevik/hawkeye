# Incident Report — event-bus — 2024-05-01

**Date:** 2024-05-01
**Severity:** P2
**Duration:** ~212 minutes
**Service:** event-bus
**Responders:** Ravi Sharma, Henrik Larsen

## Summary

Event-bus experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
212 minutes and affected 65% of traffic.

## Timeline

- **2024-05-01 09:12** — Alerts triggered on S3 metrics
- **2024-05-01 09:18** — Ravi Sharma acknowledged the alert
- **2024-05-01 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-05-01 09:41** — Mitigation applied (rolled back last deployment)
- **2024-05-01 12:44** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 44,553 requests failed
- 485 users affected
- Downstream services impacted: search-service, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for S3
- [ ] Schedule blameless post-mortem with Ravi Sharma, Henrik Larsen

## Lessons Learned

We need better integration tests to catch these issues before production.
