# Incident Report — media-uploader — 2023-07-25

**Date:** 2023-07-25
**Severity:** P3
**Duration:** ~113 minutes
**Service:** media-uploader
**Responders:** Elena Rossi, Ravi Sharma

## Summary

Media-uploader experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
113 minutes and affected 30% of traffic.

## Timeline

- **2023-07-25 09:12** — Alerts triggered on Go metrics
- **2023-07-25 09:18** — Ravi Sharma acknowledged the alert
- **2023-07-25 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2023-07-25 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-25 10:65** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 26,371 requests failed
- 152 users affected
- Downstream services impacted: data-warehouse, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Go
- [ ] Schedule blameless post-mortem with Elena Rossi, Ravi Sharma

## Lessons Learned

We need better integration tests to catch these issues before production.
