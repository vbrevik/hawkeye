# Incident Report — user-service — 2025-07-23

**Date:** 2025-07-23
**Severity:** P3
**Duration:** ~190 minutes
**Service:** user-service
**Responders:** Frank Müller, David Park, Elena Rossi

## Summary

User-service experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
190 minutes and affected 12% of traffic.

## Timeline

- **2025-07-23 09:12** — Alerts triggered on Elasticsearch metrics
- **2025-07-23 09:18** — Frank Müller acknowledged the alert
- **2025-07-23 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-07-23 09:41** — Mitigation applied (rolled back last deployment)
- **2025-07-23 12:22** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 27,364 requests failed
- 467 users affected
- Downstream services impacted: search-service, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Frank Müller, David Park

## Lessons Learned

We need better alerting coverage to catch these issues before production.
