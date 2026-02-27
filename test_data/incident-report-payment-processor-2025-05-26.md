# Incident Report — payment-processor — 2025-05-26

**Date:** 2025-05-26
**Severity:** P3
**Duration:** ~31 minutes
**Service:** payment-processor
**Responders:** Laura Bianchi, Frank Müller, Alice Chen, Priya Patel

## Summary

Payment-processor experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
31 minutes and affected 55% of traffic.

## Timeline

- **2025-05-26 09:12** — Alerts triggered on Redis metrics
- **2025-05-26 09:18** — Priya Patel acknowledged the alert
- **2025-05-26 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-05-26 09:41** — Mitigation applied (rolled back last deployment)
- **2025-05-26 09:43** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 43,258 requests failed
- 347 users affected
- Downstream services impacted: api-gateway, audit-logger

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for payment-processor

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Redis
- [ ] Schedule blameless post-mortem with Laura Bianchi, Frank Müller

## Lessons Learned

We need better canary deployments to catch these issues before production.
