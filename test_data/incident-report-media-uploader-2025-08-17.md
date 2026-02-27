# Incident Report — media-uploader — 2025-08-17

**Date:** 2025-08-17
**Severity:** P2
**Duration:** ~201 minutes
**Service:** media-uploader
**Responders:** Frank Müller, David Park

## Summary

Media-uploader experienced an outage due to race condition during concurrent writes. The incident lasted approximately
201 minutes and affected 60% of traffic.

## Timeline

- **2025-08-17 09:12** — Alerts triggered on ArgoCD metrics
- **2025-08-17 09:18** — Frank Müller acknowledged the alert
- **2025-08-17 09:25** — Root cause identified: race condition during concurrent writes
- **2025-08-17 09:41** — Mitigation applied (rolled back last deployment)
- **2025-08-17 12:33** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
review and rotate all secrets in Vault. The change was not caught in staging because the
load pattern was different.

## Impact

- 21,643 requests failed
- 34 users affected
- Downstream services impacted: analytics-pipeline, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for ArgoCD
- [ ] Schedule blameless post-mortem with Frank Müller, David Park

## Lessons Learned

We need better alerting coverage to catch these issues before production.
