# Incident Report — event-bus — 2025-02-22

**Date:** 2025-02-22
**Severity:** P1
**Duration:** ~142 minutes
**Service:** event-bus
**Responders:** Laura Bianchi, David Park

## Summary

Event-bus experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
142 minutes and affected 55% of traffic.

## Timeline

- **2025-02-22 09:12** — Alerts triggered on Elasticsearch metrics
- **2025-02-22 09:18** — Laura Bianchi acknowledged the alert
- **2025-02-22 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-02-22 09:41** — Mitigation applied (rolled back last deployment)
- **2025-02-22 11:34** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 327 requests failed
- 388 users affected
- Downstream services impacted: auth-service, user-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for event-bus

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Laura Bianchi, David Park

## Lessons Learned

We need better integration tests to catch these issues before production.
