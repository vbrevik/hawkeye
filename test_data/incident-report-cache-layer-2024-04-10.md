# Incident Report — cache-layer — 2024-04-10

**Date:** 2024-04-10
**Severity:** P2
**Duration:** ~81 minutes
**Service:** cache-layer
**Responders:** Ravi Sharma, Priya Patel, Henrik Larsen

## Summary

Cache-layer experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
81 minutes and affected 15% of traffic.

## Timeline

- **2024-04-10 09:12** — Alerts triggered on PostgreSQL metrics
- **2024-04-10 09:18** — Priya Patel acknowledged the alert
- **2024-04-10 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2024-04-10 09:41** — Mitigation applied (rolled back last deployment)
- **2024-04-10 10:33** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
refactor the authentication middleware. The change was not caught in staging because the
load pattern was different.

## Impact

- 33,005 requests failed
- 293 users affected
- Downstream services impacted: audit-logger, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for PostgreSQL
- [ ] Schedule blameless post-mortem with Ravi Sharma, Priya Patel

## Lessons Learned

We need better alerting coverage to catch these issues before production.
