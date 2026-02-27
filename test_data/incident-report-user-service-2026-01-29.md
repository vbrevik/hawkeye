# Incident Report — user-service — 2026-01-29

**Date:** 2026-01-29
**Severity:** P2
**Duration:** ~61 minutes
**Service:** user-service
**Responders:** Jae-won Kim, Kofi Mensah, Bob Martins, Alice Chen

## Summary

User-service experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
61 minutes and affected 40% of traffic.

## Timeline

- **2026-01-29 09:12** — Alerts triggered on FastAPI metrics
- **2026-01-29 09:18** — Kofi Mensah acknowledged the alert
- **2026-01-29 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2026-01-29 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-29 10:13** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 1,124 requests failed
- 276 users affected
- Downstream services impacted: media-uploader, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Jae-won Kim, Kofi Mensah

## Lessons Learned

We need better alerting coverage to catch these issues before production.
