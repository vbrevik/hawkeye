# Incident Report — report-generator — 2026-02-07

**Date:** 2026-02-07
**Severity:** P3
**Duration:** ~16 minutes
**Service:** report-generator
**Responders:** Alice Chen, Laura Bianchi, Ravi Sharma

## Summary

Report-generator experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
16 minutes and affected 81% of traffic.

## Timeline

- **2026-02-07 09:12** — Alerts triggered on React metrics
- **2026-02-07 09:18** — Alice Chen acknowledged the alert
- **2026-02-07 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2026-02-07 09:41** — Mitigation applied (rolled back last deployment)
- **2026-02-07 09:28** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 7,254 requests failed
- 40 users affected
- Downstream services impacted: cache-layer, event-bus

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for React
- [ ] Schedule blameless post-mortem with Alice Chen, Laura Bianchi

## Lessons Learned

We need better staging parity to catch these issues before production.
