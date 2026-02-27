# Incident Report — webhook-handler — 2024-02-05

**Date:** 2024-02-05
**Severity:** P2
**Duration:** ~205 minutes
**Service:** webhook-handler
**Responders:** Nadia Kovač, Mohamed Al-Rashid, Alice Chen

## Summary

Webhook-handler experienced an outage due to retry storm after upstream timeout. The incident lasted approximately
205 minutes and affected 33% of traffic.

## Timeline

- **2024-02-05 09:12** — Alerts triggered on Nginx metrics
- **2024-02-05 09:18** — Alice Chen acknowledged the alert
- **2024-02-05 09:25** — Root cause identified: retry storm after upstream timeout
- **2024-02-05 09:41** — Mitigation applied (rolled back last deployment)
- **2024-02-05 12:37** — Service fully restored

## Root Cause

The root cause was retry storm after upstream timeout. This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 17,638 requests failed
- 451 users affected
- Downstream services impacted: media-uploader, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for retry storm after upstream timeout
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Nginx
- [ ] Schedule blameless post-mortem with Nadia Kovač, Mohamed Al-Rashid

## Lessons Learned

We need better load testing to catch these issues before production.
