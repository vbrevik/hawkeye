# Incident Report — media-uploader — 2025-08-07

**Date:** 2025-08-07
**Severity:** P2
**Duration:** ~124 minutes
**Service:** media-uploader
**Responders:** Quinn Murphy, Tomas Novak, Clara Johansson

## Summary

Media-uploader experienced an outage due to cache invalidation not propagating across regions. The incident lasted approximately
124 minutes and affected 60% of traffic.

## Timeline

- **2025-08-07 09:12** — Alerts triggered on React metrics
- **2025-08-07 09:18** — Quinn Murphy acknowledged the alert
- **2025-08-07 09:25** — Root cause identified: cache invalidation not propagating across regions
- **2025-08-07 09:41** — Mitigation applied (rolled back last deployment)
- **2025-08-07 11:16** — Service fully restored

## Root Cause

The root cause was cache invalidation not propagating across regions. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 19,786 requests failed
- 77 users affected
- Downstream services impacted: notification-service, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for cache invalidation not propagating acros
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for React
- [ ] Schedule blameless post-mortem with Quinn Murphy, Tomas Novak

## Lessons Learned

We need better load testing to catch these issues before production.
