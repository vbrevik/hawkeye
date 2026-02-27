# Incident Report — webhook-handler — 2025-11-15

**Date:** 2025-11-15
**Severity:** P3
**Duration:** ~54 minutes
**Service:** webhook-handler
**Responders:** Kofi Mensah, Elena Rossi, Laura Bianchi, Isabelle Dupont

## Summary

Webhook-handler experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
54 minutes and affected 67% of traffic.

## Timeline

- **2025-11-15 09:12** — Alerts triggered on SQLite metrics
- **2025-11-15 09:18** — Laura Bianchi acknowledged the alert
- **2025-11-15 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2025-11-15 09:41** — Mitigation applied (rolled back last deployment)
- **2025-11-15 09:66** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 5,559 requests failed
- 468 users affected
- Downstream services impacted: event-bus, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for SQLite
- [ ] Schedule blameless post-mortem with Kofi Mensah, Elena Rossi

## Lessons Learned

We need better staging parity to catch these issues before production.
