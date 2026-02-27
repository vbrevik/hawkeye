# Incident Report — cache-layer — 2025-08-18

**Date:** 2025-08-18
**Severity:** P2
**Duration:** ~157 minutes
**Service:** cache-layer
**Responders:** Jae-won Kim, Alice Chen, Nadia Kovač

## Summary

Cache-layer experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
157 minutes and affected 34% of traffic.

## Timeline

- **2025-08-18 09:12** — Alerts triggered on Kubernetes metrics
- **2025-08-18 09:18** — Jae-won Kim acknowledged the alert
- **2025-08-18 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-08-18 09:41** — Mitigation applied (rolled back last deployment)
- **2025-08-18 11:49** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 24,587 requests failed
- 97 users affected
- Downstream services impacted: data-warehouse, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Jae-won Kim, Alice Chen

## Lessons Learned

We need better integration tests to catch these issues before production.
