# Incident Report — search-service — 2023-03-12

**Date:** 2023-03-12
**Severity:** P2
**Duration:** ~79 minutes
**Service:** search-service
**Responders:** David Park, Isabelle Dupont, Priya Patel, Gina Torres

## Summary

Search-service experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
79 minutes and affected 95% of traffic.

## Timeline

- **2023-03-12 09:12** — Alerts triggered on Prometheus metrics
- **2023-03-12 09:18** — Gina Torres acknowledged the alert
- **2023-03-12 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2023-03-12 09:41** — Mitigation applied (rolled back last deployment)
- **2023-03-12 10:31** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 21,932 requests failed
- 79 users affected
- Downstream services impacted: webhook-handler, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for search-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Prometheus
- [ ] Schedule blameless post-mortem with David Park, Isabelle Dupont

## Lessons Learned

We need better canary deployments to catch these issues before production.
