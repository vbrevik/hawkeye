# Incident Report — scheduler — 2024-04-28

**Date:** 2024-04-28
**Severity:** P2
**Duration:** ~175 minutes
**Service:** scheduler
**Responders:** Ravi Sharma, Henrik Larsen, Laura Bianchi, Jae-won Kim

## Summary

Scheduler experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
175 minutes and affected 78% of traffic.

## Timeline

- **2024-04-28 09:12** — Alerts triggered on FastAPI metrics
- **2024-04-28 09:18** — Henrik Larsen acknowledged the alert
- **2024-04-28 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2024-04-28 09:41** — Mitigation applied (rolled back last deployment)
- **2024-04-28 11:67** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 34,156 requests failed
- 378 users affected
- Downstream services impacted: payment-processor, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Ravi Sharma, Henrik Larsen

## Lessons Learned

We need better load testing to catch these issues before production.
