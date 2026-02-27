# Incident Report — event-bus — 2024-01-29

**Date:** 2024-01-29
**Severity:** P1
**Duration:** ~231 minutes
**Service:** event-bus
**Responders:** Henrik Larsen, Sofia Andersen, Kofi Mensah, Oscar Lindberg

## Summary

Event-bus experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
231 minutes and affected 42% of traffic.

## Timeline

- **2024-01-29 09:12** — Alerts triggered on Grafana metrics
- **2024-01-29 09:18** — Oscar Lindberg acknowledged the alert
- **2024-01-29 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2024-01-29 09:41** — Mitigation applied (rolled back last deployment)
- **2024-01-29 12:63** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 44,212 requests failed
- 335 users affected
- Downstream services impacted: report-generator, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Grafana
- [ ] Schedule blameless post-mortem with Henrik Larsen, Sofia Andersen

## Lessons Learned

We need better integration tests to catch these issues before production.
