# Incident Report — user-service — 2023-10-06

**Date:** 2023-10-06
**Severity:** P2
**Duration:** ~79 minutes
**Service:** user-service
**Responders:** Kofi Mensah, Frank Müller, Mohamed Al-Rashid

## Summary

User-service experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
79 minutes and affected 93% of traffic.

## Timeline

- **2023-10-06 09:12** — Alerts triggered on Prometheus metrics
- **2023-10-06 09:18** — Mohamed Al-Rashid acknowledged the alert
- **2023-10-06 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2023-10-06 09:41** — Mitigation applied (rolled back last deployment)
- **2023-10-06 10:31** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 37,840 requests failed
- 401 users affected
- Downstream services impacted: payment-processor, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Prometheus
- [ ] Schedule blameless post-mortem with Kofi Mensah, Frank Müller

## Lessons Learned

We need better integration tests to catch these issues before production.
