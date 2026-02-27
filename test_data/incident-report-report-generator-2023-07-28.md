# Incident Report — report-generator — 2023-07-28

**Date:** 2023-07-28
**Severity:** P3
**Duration:** ~221 minutes
**Service:** report-generator
**Responders:** Kofi Mensah, Gina Torres

## Summary

Report-generator experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
221 minutes and affected 56% of traffic.

## Timeline

- **2023-07-28 09:12** — Alerts triggered on Go metrics
- **2023-07-28 09:18** — Kofi Mensah acknowledged the alert
- **2023-07-28 09:25** — Root cause identified: retry storm after upstream timeout
- **2023-07-28 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-28 12:53** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 27,933 requests failed
- 372 users affected
- Downstream services impacted: event-bus, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Kofi Mensah, Gina Torres

## Lessons Learned

We need better alerting coverage to catch these issues before production.
