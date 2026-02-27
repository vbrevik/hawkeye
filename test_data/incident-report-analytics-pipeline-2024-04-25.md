# Incident Report — analytics-pipeline — 2024-04-25

**Date:** 2024-04-25
**Severity:** P1
**Duration:** ~239 minutes
**Service:** analytics-pipeline
**Responders:** Elena Rossi, Kofi Mensah, Henrik Larsen, Tomas Novak

## Summary

Analytics-pipeline experienced an outage due to race condition during concurrent writes. The incident lasted approximately
239 minutes and affected 69% of traffic.

## Timeline

- **2024-04-25 09:12** — Alerts triggered on ArgoCD metrics
- **2024-04-25 09:18** — Tomas Novak acknowledged the alert
- **2024-04-25 09:25** — Root cause identified: race condition during concurrent writes
- **2024-04-25 09:41** — Mitigation applied (rolled back last deployment)
- **2024-04-25 12:71** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 20,644 requests failed
- 54 users affected
- Downstream services impacted: scheduler, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for ArgoCD
- [ ] Schedule blameless post-mortem with Elena Rossi, Kofi Mensah

## Lessons Learned

We need better alerting coverage to catch these issues before production.
