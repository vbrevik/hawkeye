# Incident Report — media-uploader — 2023-07-27

**Date:** 2023-07-27
**Severity:** P3
**Duration:** ~134 minutes
**Service:** media-uploader
**Responders:** Kofi Mensah, Elena Rossi

## Summary

Media-uploader experienced an outage due to memory leak in the worker pool. The incident lasted approximately
134 minutes and affected 54% of traffic.

## Timeline

- **2023-07-27 09:12** — Alerts triggered on Elasticsearch metrics
- **2023-07-27 09:18** — Elena Rossi acknowledged the alert
- **2023-07-27 09:25** — Root cause identified: memory leak in the worker pool
- **2023-07-27 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-27 11:26** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 31,959 requests failed
- 335 users affected
- Downstream services impacted: media-uploader, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Kofi Mensah, Elena Rossi

## Lessons Learned

We need better integration tests to catch these issues before production.
