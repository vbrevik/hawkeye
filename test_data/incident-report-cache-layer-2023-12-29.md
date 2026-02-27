# Incident Report — cache-layer — 2023-12-29

**Date:** 2023-12-29
**Severity:** P3
**Duration:** ~77 minutes
**Service:** cache-layer
**Responders:** Mohamed Al-Rashid, Jae-won Kim, Nadia Kovač

## Summary

Cache-layer experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
77 minutes and affected 94% of traffic.

## Timeline

- **2023-12-29 09:12** — Alerts triggered on S3 metrics
- **2023-12-29 09:18** — Nadia Kovač acknowledged the alert
- **2023-12-29 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2023-12-29 09:41** — Mitigation applied (rolled back last deployment)
- **2023-12-29 10:29** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 5,842 requests failed
- 413 users affected
- Downstream services impacted: webhook-handler, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for S3
- [ ] Schedule blameless post-mortem with Mohamed Al-Rashid, Jae-won Kim

## Lessons Learned

We need better staging parity to catch these issues before production.
