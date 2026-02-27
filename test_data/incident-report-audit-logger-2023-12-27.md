# Incident Report — audit-logger — 2023-12-27

**Date:** 2023-12-27
**Severity:** P1
**Duration:** ~81 minutes
**Service:** audit-logger
**Responders:** Jae-won Kim, Bob Martins

## Summary

Audit-logger experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
81 minutes and affected 77% of traffic.

## Timeline

- **2023-12-27 09:12** — Alerts triggered on PostgreSQL metrics
- **2023-12-27 09:18** — Jae-won Kim acknowledged the alert
- **2023-12-27 09:25** — Root cause identified: flaky tests in the integration suite
- **2023-12-27 09:41** — Mitigation applied (rolled back last deployment)
- **2023-12-27 10:33** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 18,749 requests failed
- 233 users affected
- Downstream services impacted: media-uploader, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for audit-logger

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for PostgreSQL
- [ ] Schedule blameless post-mortem with Jae-won Kim, Bob Martins

## Lessons Learned

We need better alerting coverage to catch these issues before production.
