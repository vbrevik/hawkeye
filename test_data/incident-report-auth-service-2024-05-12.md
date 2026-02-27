# Incident Report — auth-service — 2024-05-12

**Date:** 2024-05-12
**Severity:** P1
**Duration:** ~45 minutes
**Service:** auth-service
**Responders:** Tomas Novak, Elena Rossi, Laura Bianchi, Clara Johansson

## Summary

Auth-service experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
45 minutes and affected 68% of traffic.

## Timeline

- **2024-05-12 09:12** — Alerts triggered on PostgreSQL metrics
- **2024-05-12 09:18** — Tomas Novak acknowledged the alert
- **2024-05-12 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-05-12 09:41** — Mitigation applied (rolled back last deployment)
- **2024-05-12 09:57** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 22,278 requests failed
- 477 users affected
- Downstream services impacted: scheduler, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for auth-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for PostgreSQL
- [ ] Schedule blameless post-mortem with Tomas Novak, Elena Rossi

## Lessons Learned

We need better staging parity to catch these issues before production.
