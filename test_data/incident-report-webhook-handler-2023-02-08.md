# Incident Report — webhook-handler — 2023-02-08

**Date:** 2023-02-08
**Severity:** P1
**Duration:** ~173 minutes
**Service:** webhook-handler
**Responders:** Gina Torres, Mohamed Al-Rashid

## Summary

Webhook-handler experienced an outage due to memory leak in the worker pool. The incident lasted approximately
173 minutes and affected 60% of traffic.

## Timeline

- **2023-02-08 09:12** — Alerts triggered on Elasticsearch metrics
- **2023-02-08 09:18** — Gina Torres acknowledged the alert
- **2023-02-08 09:25** — Root cause identified: memory leak in the worker pool
- **2023-02-08 09:41** — Mitigation applied (rolled back last deployment)
- **2023-02-08 11:65** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 2,151 requests failed
- 278 users affected
- Downstream services impacted: scheduler, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Gina Torres, Mohamed Al-Rashid

## Lessons Learned

We need better staging parity to catch these issues before production.
