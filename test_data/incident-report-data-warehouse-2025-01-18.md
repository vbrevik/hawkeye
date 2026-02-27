# Incident Report — data-warehouse — 2025-01-18

**Date:** 2025-01-18
**Severity:** P2
**Duration:** ~217 minutes
**Service:** data-warehouse
**Responders:** David Park, Henrik Larsen

## Summary

Data-warehouse experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
217 minutes and affected 19% of traffic.

## Timeline

- **2025-01-18 09:12** — Alerts triggered on DynamoDB metrics
- **2025-01-18 09:18** — David Park acknowledged the alert
- **2025-01-18 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-01-18 09:41** — Mitigation applied (rolled back last deployment)
- **2025-01-18 12:49** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 46,807 requests failed
- 152 users affected
- Downstream services impacted: notification-service, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for DynamoDB
- [ ] Schedule blameless post-mortem with David Park, Henrik Larsen

## Lessons Learned

We need better load testing to catch these issues before production.
