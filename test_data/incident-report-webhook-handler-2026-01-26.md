# Incident Report — webhook-handler — 2026-01-26

**Date:** 2026-01-26
**Severity:** P3
**Duration:** ~93 minutes
**Service:** webhook-handler
**Responders:** Gina Torres, Tomas Novak, Sofia Andersen, Jae-won Kim

## Summary

Webhook-handler experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
93 minutes and affected 91% of traffic.

## Timeline

- **2026-01-26 09:12** — Alerts triggered on Prometheus metrics
- **2026-01-26 09:18** — Tomas Novak acknowledged the alert
- **2026-01-26 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2026-01-26 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-26 10:45** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 33,123 requests failed
- 278 users affected
- Downstream services impacted: report-generator, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for webhook-handler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Prometheus
- [ ] Schedule blameless post-mortem with Gina Torres, Tomas Novak

## Lessons Learned

We need better load testing to catch these issues before production.
