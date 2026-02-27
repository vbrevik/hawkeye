# Incident Report — audit-logger — 2025-12-01

**Date:** 2025-12-01
**Severity:** P3
**Duration:** ~83 minutes
**Service:** audit-logger
**Responders:** Bob Martins, Henrik Larsen

## Summary

Audit-logger experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
83 minutes and affected 38% of traffic.

## Timeline

- **2025-12-01 09:12** — Alerts triggered on Kubernetes metrics
- **2025-12-01 09:18** — Henrik Larsen acknowledged the alert
- **2025-12-01 09:25** — Root cause identified: retry storm after upstream timeout
- **2025-12-01 09:41** — Mitigation applied (rolled back last deployment)
- **2025-12-01 10:35** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 22,219 requests failed
- 21 users affected
- Downstream services impacted: user-service, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for audit-logger

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Bob Martins, Henrik Larsen

## Lessons Learned

We need better integration tests to catch these issues before production.
