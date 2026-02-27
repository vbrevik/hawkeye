# Incident Report — media-uploader — 2024-01-07

**Date:** 2024-01-07
**Severity:** P3
**Duration:** ~38 minutes
**Service:** media-uploader
**Responders:** Gina Torres, Alice Chen, Nadia Kovač

## Summary

Media-uploader experienced an outage due to memory leak in the worker pool. The incident lasted approximately
38 minutes and affected 40% of traffic.

## Timeline

- **2024-01-07 09:12** — Alerts triggered on Docker metrics
- **2024-01-07 09:18** — Alice Chen acknowledged the alert
- **2024-01-07 09:25** — Root cause identified: memory leak in the worker pool
- **2024-01-07 09:41** — Mitigation applied (rolled back last deployment)
- **2024-01-07 09:50** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 39,361 requests failed
- 405 users affected
- Downstream services impacted: media-uploader, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Docker
- [ ] Schedule blameless post-mortem with Gina Torres, Alice Chen

## Lessons Learned

We need better staging parity to catch these issues before production.
