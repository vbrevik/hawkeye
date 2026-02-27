# Incident Report — cache-layer — 2024-09-20

**Date:** 2024-09-20
**Severity:** P2
**Duration:** ~217 minutes
**Service:** cache-layer
**Responders:** Oscar Lindberg, Nadia Kovač, Sofia Andersen, Laura Bianchi

## Summary

Cache-layer experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
217 minutes and affected 74% of traffic.

## Timeline

- **2024-09-20 09:12** — Alerts triggered on Kafka metrics
- **2024-09-20 09:18** — Laura Bianchi acknowledged the alert
- **2024-09-20 09:25** — Root cause identified: flaky tests in the integration suite
- **2024-09-20 09:41** — Mitigation applied (rolled back last deployment)
- **2024-09-20 12:49** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 22,108 requests failed
- 59 users affected
- Downstream services impacted: audit-logger, notification-service

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kafka
- [ ] Schedule blameless post-mortem with Oscar Lindberg, Nadia Kovač

## Lessons Learned

We need better integration tests to catch these issues before production.
