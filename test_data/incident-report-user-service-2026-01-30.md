# Incident Report — user-service — 2026-01-30

**Date:** 2026-01-30
**Severity:** P2
**Duration:** ~175 minutes
**Service:** user-service
**Responders:** Priya Patel, Nadia Kovač, Sofia Andersen, Frank Müller

## Summary

User-service experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
175 minutes and affected 12% of traffic.

## Timeline

- **2026-01-30 09:12** — Alerts triggered on React metrics
- **2026-01-30 09:18** — Sofia Andersen acknowledged the alert
- **2026-01-30 09:25** — Root cause identified: flaky tests in the integration suite
- **2026-01-30 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-30 11:67** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 4,484 requests failed
- 488 users affected
- Downstream services impacted: analytics-pipeline, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for React
- [ ] Schedule blameless post-mortem with Priya Patel, Nadia Kovač

## Lessons Learned

We need better integration tests to catch these issues before production.
