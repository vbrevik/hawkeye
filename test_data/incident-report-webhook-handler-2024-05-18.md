# Incident Report — webhook-handler — 2024-05-18

**Date:** 2024-05-18
**Severity:** P3
**Duration:** ~29 minutes
**Service:** webhook-handler
**Responders:** Tomas Novak, Quinn Murphy, Bob Martins

## Summary

Webhook-handler experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
29 minutes and affected 13% of traffic.

## Timeline

- **2024-05-18 09:12** — Alerts triggered on FastAPI metrics
- **2024-05-18 09:18** — Tomas Novak acknowledged the alert
- **2024-05-18 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-05-18 09:41** — Mitigation applied (rolled back last deployment)
- **2024-05-18 09:41** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 668 requests failed
- 472 users affected
- Downstream services impacted: webhook-handler, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Tomas Novak, Quinn Murphy

## Lessons Learned

We need better staging parity to catch these issues before production.
