# Incident Report — media-uploader — 2025-07-17

**Date:** 2025-07-17
**Severity:** P3
**Duration:** ~208 minutes
**Service:** media-uploader
**Responders:** Kofi Mensah, Bob Martins, Nadia Kovač, David Park

## Summary

Media-uploader experienced an outage due to race condition during concurrent writes. The incident lasted approximately
208 minutes and affected 71% of traffic.

## Timeline

- **2025-07-17 09:12** — Alerts triggered on FastAPI metrics
- **2025-07-17 09:18** — Nadia Kovač acknowledged the alert
- **2025-07-17 09:25** — Root cause identified: race condition during concurrent writes
- **2025-07-17 09:41** — Mitigation applied (rolled back last deployment)
- **2025-07-17 12:40** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 36,680 requests failed
- 339 users affected
- Downstream services impacted: webhook-handler, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Kofi Mensah, Bob Martins

## Lessons Learned

We need better staging parity to catch these issues before production.
