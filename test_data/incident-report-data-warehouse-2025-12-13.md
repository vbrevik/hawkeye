# Incident Report — data-warehouse — 2025-12-13

**Date:** 2025-12-13
**Severity:** P3
**Duration:** ~113 minutes
**Service:** data-warehouse
**Responders:** Isabelle Dupont, Sofia Andersen, Priya Patel

## Summary

Data-warehouse experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
113 minutes and affected 61% of traffic.

## Timeline

- **2025-12-13 09:12** — Alerts triggered on Helm metrics
- **2025-12-13 09:18** — Priya Patel acknowledged the alert
- **2025-12-13 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-12-13 09:41** — Mitigation applied (rolled back last deployment)
- **2025-12-13 10:65** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 48,053 requests failed
- 499 users affected
- Downstream services impacted: data-warehouse, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Helm
- [ ] Schedule blameless post-mortem with Isabelle Dupont, Sofia Andersen

## Lessons Learned

We need better canary deployments to catch these issues before production.
