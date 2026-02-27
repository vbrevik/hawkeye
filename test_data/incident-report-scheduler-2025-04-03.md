# Incident Report — scheduler — 2025-04-03

**Date:** 2025-04-03
**Severity:** P1
**Duration:** ~150 minutes
**Service:** scheduler
**Responders:** Sofia Andersen, David Park, Kofi Mensah, Isabelle Dupont

## Summary

Scheduler experienced an outage due to race condition during concurrent writes. The incident lasted approximately
150 minutes and affected 72% of traffic.

## Timeline

- **2025-04-03 09:12** — Alerts triggered on React metrics
- **2025-04-03 09:18** — Isabelle Dupont acknowledged the alert
- **2025-04-03 09:25** — Root cause identified: race condition during concurrent writes
- **2025-04-03 09:41** — Mitigation applied (rolled back last deployment)
- **2025-04-03 11:42** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 13,845 requests failed
- 178 users affected
- Downstream services impacted: scheduler, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for React
- [ ] Schedule blameless post-mortem with Sofia Andersen, David Park

## Lessons Learned

We need better load testing to catch these issues before production.
