# Incident Report — report-generator — 2025-05-08

**Date:** 2025-05-08
**Severity:** P3
**Duration:** ~38 minutes
**Service:** report-generator
**Responders:** Isabelle Dupont, Oscar Lindberg, Frank Müller

## Summary

Report-generator experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
38 minutes and affected 78% of traffic.

## Timeline

- **2025-05-08 09:12** — Alerts triggered on Axum metrics
- **2025-05-08 09:18** — Frank Müller acknowledged the alert
- **2025-05-08 09:25** — Root cause identified: retry storm after upstream timeout
- **2025-05-08 09:41** — Mitigation applied (rolled back last deployment)
- **2025-05-08 09:50** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 6,645 requests failed
- 370 users affected
- Downstream services impacted: user-service, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Axum
- [ ] Schedule blameless post-mortem with Isabelle Dupont, Oscar Lindberg

## Lessons Learned

We need better canary deployments to catch these issues before production.
