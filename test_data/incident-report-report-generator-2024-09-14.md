# Incident Report — report-generator — 2024-09-14

**Date:** 2024-09-14
**Severity:** P2
**Duration:** ~36 minutes
**Service:** report-generator
**Responders:** Bob Martins, Laura Bianchi, Isabelle Dupont, Quinn Murphy

## Summary

Report-generator experienced an outage due to race condition during concurrent writes. The incident lasted approximately
36 minutes and affected 11% of traffic.

## Timeline

- **2024-09-14 09:12** — Alerts triggered on gRPC metrics
- **2024-09-14 09:18** — Bob Martins acknowledged the alert
- **2024-09-14 09:25** — Root cause identified: race condition during concurrent writes
- **2024-09-14 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-14 09:48** — Service fully restored

## Root Cause

The root cause was race condition during concurrent writes. This was introduced in the latest release when
add structured logging with trace IDs. The change was not caught in staging because the
load pattern was different.

## Impact

- 19,627 requests failed
- 294 users affected
- Downstream services impacted: report-generator, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for race condition during concurrent writes
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for gRPC
- [ ] Schedule blameless post-mortem with Bob Martins, Laura Bianchi

## Lessons Learned

We need better integration tests to catch these issues before production.
