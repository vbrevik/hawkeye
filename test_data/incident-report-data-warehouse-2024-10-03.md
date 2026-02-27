# Incident Report — data-warehouse — 2024-10-03

**Date:** 2024-10-03
**Severity:** P1
**Duration:** ~221 minutes
**Service:** data-warehouse
**Responders:** Bob Martins, Sofia Andersen, Ravi Sharma

## Summary

Data-warehouse experienced an outage due to memory leak in the worker pool. The incident lasted approximately
221 minutes and affected 54% of traffic.

## Timeline

- **2024-10-03 09:12** — Alerts triggered on Helm metrics
- **2024-10-03 09:18** — Sofia Andersen acknowledged the alert
- **2024-10-03 09:25** — Root cause identified: memory leak in the worker pool
- **2024-10-03 09:41** — Mitigation applied (rolled back last deployment)
- **2024-10-03 12:53** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 24,022 requests failed
- 409 users affected
- Downstream services impacted: media-uploader, scheduler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Helm
- [ ] Schedule blameless post-mortem with Bob Martins, Sofia Andersen

## Lessons Learned

We need better canary deployments to catch these issues before production.
