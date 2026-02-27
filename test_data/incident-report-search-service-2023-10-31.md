# Incident Report — search-service — 2023-10-31

**Date:** 2023-10-31
**Severity:** P1
**Duration:** ~35 minutes
**Service:** search-service
**Responders:** Oscar Lindberg, Alice Chen

## Summary

Search-service experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
35 minutes and affected 58% of traffic.

## Timeline

- **2023-10-31 09:12** — Alerts triggered on Nginx metrics
- **2023-10-31 09:18** — Alice Chen acknowledged the alert
- **2023-10-31 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2023-10-31 09:41** — Mitigation applied (rolled back last deployment)
- **2023-10-31 09:47** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 46,721 requests failed
- 477 users affected
- Downstream services impacted: event-bus, report-generator

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for search-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Nginx
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Alice Chen

## Lessons Learned

We need better staging parity to catch these issues before production.
