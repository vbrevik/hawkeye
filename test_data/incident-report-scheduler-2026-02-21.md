# Incident Report — scheduler — 2026-02-21

**Date:** 2026-02-21
**Severity:** P2
**Duration:** ~81 minutes
**Service:** scheduler
**Responders:** Gina Torres, Jae-won Kim, Isabelle Dupont, Clara Johansson

## Summary

Scheduler experienced an outage due to memory leak in the worker pool. The incident lasted approximately
81 minutes and affected 69% of traffic.

## Timeline

- **2026-02-21 09:12** — Alerts triggered on DynamoDB metrics
- **2026-02-21 09:18** — Isabelle Dupont acknowledged the alert
- **2026-02-21 09:25** — Root cause identified: memory leak in the worker pool
- **2026-02-21 09:41** — Mitigation applied (rolled back last deployment)
- **2026-02-21 10:33** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 7,560 requests failed
- 352 users affected
- Downstream services impacted: analytics-pipeline, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Gina Torres, Jae-won Kim

## Lessons Learned

We need better integration tests to catch these issues before production.
