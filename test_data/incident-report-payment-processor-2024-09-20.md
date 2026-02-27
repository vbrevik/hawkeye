# Incident Report — payment-processor — 2024-09-20

**Date:** 2024-09-20
**Severity:** P2
**Duration:** ~236 minutes
**Service:** payment-processor
**Responders:** Ravi Sharma, Oscar Lindberg

## Summary

Payment-processor experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
236 minutes and affected 84% of traffic.

## Timeline

- **2024-09-20 09:12** — Alerts triggered on Elasticsearch metrics
- **2024-09-20 09:18** — Oscar Lindberg acknowledged the alert
- **2024-09-20 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-09-20 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-20 12:68** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 13,168 requests failed
- 112 users affected
- Downstream services impacted: analytics-pipeline, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for payment-processor

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Ravi Sharma, Oscar Lindberg

## Lessons Learned

We need better canary deployments to catch these issues before production.
