# Incident Report — user-service — 2025-12-29

**Date:** 2025-12-29
**Severity:** P2
**Duration:** ~98 minutes
**Service:** user-service
**Responders:** Clara Johansson, Oscar Lindberg

## Summary

User-service experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
98 minutes and affected 57% of traffic.

## Timeline

- **2025-12-29 09:12** — Alerts triggered on Grafana metrics
- **2025-12-29 09:18** — Oscar Lindberg acknowledged the alert
- **2025-12-29 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-12-29 09:41** — Mitigation applied (rolled back last deployment)
- **2025-12-29 10:50** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 19,994 requests failed
- 6 users affected
- Downstream services impacted: auth-service, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for user-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Grafana
- [ ] Schedule blameless post-mortem with Clara Johansson, Oscar Lindberg

## Lessons Learned

We need better canary deployments to catch these issues before production.
