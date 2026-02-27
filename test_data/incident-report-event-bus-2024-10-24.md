# Incident Report — event-bus — 2024-10-24

**Date:** 2024-10-24
**Severity:** P2
**Duration:** ~182 minutes
**Service:** event-bus
**Responders:** Gina Torres, Oscar Lindberg, Clara Johansson, Isabelle Dupont

## Summary

Event-bus experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
182 minutes and affected 88% of traffic.

## Timeline

- **2024-10-24 09:12** — Alerts triggered on Kubernetes metrics
- **2024-10-24 09:18** — Oscar Lindberg acknowledged the alert
- **2024-10-24 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-10-24 09:41** — Mitigation applied (rolled back last deployment)
- **2024-10-24 12:14** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,076 requests failed
- 59 users affected
- Downstream services impacted: report-generator, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Gina Torres, Oscar Lindberg

## Lessons Learned

We need better load testing to catch these issues before production.
