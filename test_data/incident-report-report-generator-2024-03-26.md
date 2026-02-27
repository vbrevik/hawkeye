# Incident Report — report-generator — 2024-03-26

**Date:** 2024-03-26
**Severity:** P2
**Duration:** ~168 minutes
**Service:** report-generator
**Responders:** Frank Müller, Gina Torres

## Summary

Report-generator experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
168 minutes and affected 41% of traffic.

## Timeline

- **2024-03-26 09:12** — Alerts triggered on Celery metrics
- **2024-03-26 09:18** — Gina Torres acknowledged the alert
- **2024-03-26 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-03-26 09:41** — Mitigation applied (rolled back last deployment)
- **2024-03-26 11:60** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,276 requests failed
- 95 users affected
- Downstream services impacted: cache-layer, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Celery
- [ ] Schedule blameless post-mortem with Frank Müller, Gina Torres

## Lessons Learned

We need better alerting coverage to catch these issues before production.
