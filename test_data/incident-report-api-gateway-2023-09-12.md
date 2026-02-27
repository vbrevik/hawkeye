# Incident Report — api-gateway — 2023-09-12

**Date:** 2023-09-12
**Severity:** P2
**Duration:** ~173 minutes
**Service:** api-gateway
**Responders:** Frank Müller, Laura Bianchi

## Summary

Api-gateway experienced an outage due to disk I/O bottleneck during bulk import. The incident lasted approximately
173 minutes and affected 48% of traffic.

## Timeline

- **2023-09-12 09:12** — Alerts triggered on Rust metrics
- **2023-09-12 09:18** — Frank Müller acknowledged the alert
- **2023-09-12 09:25** — Root cause identified: disk I/O bottleneck during bulk import
- **2023-09-12 09:41** — Mitigation applied (rolled back last deployment)
- **2023-09-12 11:65** — Service fully restored

## Root Cause

The root cause was disk I/O bottleneck during bulk import. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 31,727 requests failed
- 108 users affected
- Downstream services impacted: user-service, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for disk I/O bottleneck during bulk import
3. Updated runbook for api-gateway

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Rust
- [ ] Schedule blameless post-mortem with Frank Müller, Laura Bianchi

## Lessons Learned

We need better load testing to catch these issues before production.
