# Incident Report — media-uploader — 2023-05-09

**Date:** 2023-05-09
**Severity:** P2
**Duration:** ~68 minutes
**Service:** media-uploader
**Responders:** Nadia Kovač, Alice Chen, Kofi Mensah

## Summary

Media-uploader experienced an outage due to memory leak in the worker pool. The incident lasted approximately
68 minutes and affected 18% of traffic.

## Timeline

- **2023-05-09 09:12** — Alerts triggered on DynamoDB metrics
- **2023-05-09 09:18** — Alice Chen acknowledged the alert
- **2023-05-09 09:25** — Root cause identified: memory leak in the worker pool
- **2023-05-09 09:41** — Mitigation applied (rolled back last deployment)
- **2023-05-09 10:20** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 22,434 requests failed
- 427 users affected
- Downstream services impacted: auth-service, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with Nadia Kovač, Alice Chen

## Lessons Learned

We need better load testing to catch these issues before production.
