# Incident Report — audit-logger — 2023-08-29

**Date:** 2023-08-29
**Severity:** P1
**Duration:** ~158 minutes
**Service:** audit-logger
**Responders:** Quinn Murphy, Tomas Novak, Alice Chen

## Summary

Audit-logger experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
158 minutes and affected 40% of traffic.

## Timeline

- **2023-08-29 09:12** — Alerts triggered on gRPC metrics
- **2023-08-29 09:18** — Alice Chen acknowledged the alert
- **2023-08-29 09:25** — Root cause identified: flaky tests in the integration suite
- **2023-08-29 09:41** — Mitigation applied (rolled back last deployment)
- **2023-08-29 11:50** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
migrate the legacy monolith to microservices. The change was not caught in staging because the
load pattern was different.

## Impact

- 19,082 requests failed
- 24 users affected
- Downstream services impacted: search-service, auth-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for audit-logger

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for gRPC
- [ ] Schedule blameless post-mortem with Quinn Murphy, Tomas Novak

## Lessons Learned

We need better canary deployments to catch these issues before production.
