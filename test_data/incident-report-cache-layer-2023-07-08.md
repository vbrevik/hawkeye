# Incident Report — cache-layer — 2023-07-08

**Date:** 2023-07-08
**Severity:** P2
**Duration:** ~52 minutes
**Service:** cache-layer
**Responders:** Tomas Novak, Bob Martins, Henrik Larsen

## Summary

Cache-layer experienced an outage due to slow query on the user lookup table (missing index). The incident lasted approximately
52 minutes and affected 76% of traffic.

## Timeline

- **2023-07-08 09:12** — Alerts triggered on Elasticsearch metrics
- **2023-07-08 09:18** — Henrik Larsen acknowledged the alert
- **2023-07-08 09:25** — Root cause identified: slow query on the user lookup table (missing index)
- **2023-07-08 09:41** — Mitigation applied (rolled back last deployment)
- **2023-07-08 09:64** — Service fully restored

## Root Cause

The root cause was slow query on the user lookup table (missing index). This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 45,478 requests failed
- 240 users affected
- Downstream services impacted: event-bus, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for slow query on the user lookup table (mis
3. Updated runbook for cache-layer

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Tomas Novak, Bob Martins

## Lessons Learned

We need better integration tests to catch these issues before production.
