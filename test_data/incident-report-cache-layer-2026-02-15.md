# Incident Report — cache-layer — 2026-02-15

**Date:** 2026-02-15
**Severity:** P3
**Duration:** ~176 minutes
**Service:** cache-layer
**Responders:** Jae-won Kim, Quinn Murphy, Ravi Sharma, Gina Torres

## Summary

Cache-layer experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
176 minutes and affected 63% of traffic.

## Timeline

- **2026-02-15 09:12** — Alerts triggered on Kubernetes metrics
- **2026-02-15 09:18** — Ravi Sharma acknowledged the alert
- **2026-02-15 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2026-02-15 09:41** — Mitigation applied (rolled back last deployment)
- **2026-02-15 11:68** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 3,656 requests failed
- 419 users affected
- Downstream services impacted: payment-processor, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Jae-won Kim, Quinn Murphy

## Lessons Learned

We need better integration tests to catch these issues before production.
