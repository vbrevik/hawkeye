# Incident Report — media-uploader — 2024-11-23

**Date:** 2024-11-23
**Severity:** P3
**Duration:** ~178 minutes
**Service:** media-uploader
**Responders:** Kofi Mensah, David Park

## Summary

Media-uploader experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
178 minutes and affected 54% of traffic.

## Timeline

- **2024-11-23 09:12** — Alerts triggered on S3 metrics
- **2024-11-23 09:18** — David Park acknowledged the alert
- **2024-11-23 09:25** — Root cause identified: retry storm after upstream timeout
- **2024-11-23 09:41** — Mitigation applied (rolled back last deployment)
- **2024-11-23 11:70** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 47,530 requests failed
- 437 users affected
- Downstream services impacted: media-uploader, search-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for S3
- [ ] Schedule blameless post-mortem with Kofi Mensah, David Park

## Lessons Learned

We need better canary deployments to catch these issues before production.
