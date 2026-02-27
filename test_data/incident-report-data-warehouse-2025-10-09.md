# Incident Report — data-warehouse — 2025-10-09

**Date:** 2025-10-09
**Severity:** P1
**Duration:** ~109 minutes
**Service:** data-warehouse
**Responders:** Gina Torres, Jae-won Kim, Tomas Novak, Nadia Kovač

## Summary

Data-warehouse experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
109 minutes and affected 23% of traffic.

## Timeline

- **2025-10-09 09:12** — Alerts triggered on Rust metrics
- **2025-10-09 09:18** — Gina Torres acknowledged the alert
- **2025-10-09 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2025-10-09 09:41** — Mitigation applied (rolled back last deployment)
- **2025-10-09 10:61** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
write runbooks for the on-call team. The change was not caught in staging because the
load pattern was different.

## Impact

- 17,257 requests failed
- 121 users affected
- Downstream services impacted: search-service, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Rust
- [ ] Schedule blameless post-mortem with Gina Torres, Jae-won Kim

## Lessons Learned

We need better canary deployments to catch these issues before production.
