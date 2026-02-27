# Incident Report — scheduler — 2025-09-28

**Date:** 2025-09-28
**Severity:** P3
**Duration:** ~36 minutes
**Service:** scheduler
**Responders:** Clara Johansson, Ravi Sharma, Alice Chen, Oscar Lindberg

## Summary

Scheduler experienced an outage due to race condition during concurrent writes. The incident lasted approximately
36 minutes and affected 50% of traffic.

## Timeline

- **2025-09-28 09:12** — Alerts triggered on Elasticsearch metrics
- **2025-09-28 09:18** — Oscar Lindberg acknowledged the alert
- **2025-09-28 09:25** — Root cause identified: race condition during concurrent writes
- **2025-09-28 09:41** — Mitigation applied (rolled back last deployment)
- **2025-09-28 09:48** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 23,559 requests failed
- 199 users affected
- Downstream services impacted: notification-service, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Clara Johansson, Ravi Sharma

## Lessons Learned

We need better staging parity to catch these issues before production.
