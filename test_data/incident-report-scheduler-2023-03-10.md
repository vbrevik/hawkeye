# Incident Report — scheduler — 2023-03-10

**Date:** 2023-03-10
**Severity:** P1
**Duration:** ~172 minutes
**Service:** scheduler
**Responders:** Henrik Larsen, Frank Müller, Bob Martins, Laura Bianchi

## Summary

Scheduler experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
172 minutes and affected 5% of traffic.

## Timeline

- **2023-03-10 09:12** — Alerts triggered on S3 metrics
- **2023-03-10 09:18** — Frank Müller acknowledged the alert
- **2023-03-10 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2023-03-10 09:41** — Mitigation applied (rolled back last deployment)
- **2023-03-10 11:64** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 18,183 requests failed
- 400 users affected
- Downstream services impacted: data-warehouse, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for scheduler

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for S3
- [ ] Schedule blameless post-mortem with Henrik Larsen, Frank Müller

## Lessons Learned

We need better integration tests to catch these issues before production.
