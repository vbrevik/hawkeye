# Incident Report — api-gateway — 2025-01-30

**Date:** 2025-01-30
**Severity:** P2
**Duration:** ~239 minutes
**Service:** api-gateway
**Responders:** Tomas Novak, Ravi Sharma

## Summary

Api-gateway experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
239 minutes and affected 83% of traffic.

## Timeline

- **2025-01-30 09:12** — Alerts triggered on RabbitMQ metrics
- **2025-01-30 09:18** — Tomas Novak acknowledged the alert
- **2025-01-30 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-01-30 09:41** — Mitigation applied (rolled back last deployment)
- **2025-01-30 12:71** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 41,741 requests failed
- 422 users affected
- Downstream services impacted: event-bus, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for RabbitMQ
- [ ] Schedule blameless post-mortem with Tomas Novak, Ravi Sharma

## Lessons Learned

We need better load testing to catch these issues before production.
