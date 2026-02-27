# Incident Report — media-uploader — 2023-05-12

**Date:** 2023-05-12
**Severity:** P2
**Duration:** ~66 minutes
**Service:** media-uploader
**Responders:** Frank Müller, Isabelle Dupont, Henrik Larsen

## Summary

Media-uploader experienced an outage due to token expiry edge case when clock skew > 30s. The incident lasted approximately
66 minutes and affected 19% of traffic.

## Timeline

- **2023-05-12 09:12** — Alerts triggered on TypeScript metrics
- **2023-05-12 09:18** — Frank Müller acknowledged the alert
- **2023-05-12 09:25** — Root cause identified: token expiry edge case when clock skew > 30s
- **2023-05-12 09:41** — Mitigation applied (rolled back last deployment)
- **2023-05-12 10:18** — Service fully restored

## Root Cause

The root cause was token expiry edge case when clock skew > 30s. This was introduced in the latest release when
implement circuit breakers for downstream calls. The change was not caught in staging because the
load pattern was different.

## Impact

- 1,915 requests failed
- 126 users affected
- Downstream services impacted: scheduler, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for token expiry edge case when clock skew >
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for TypeScript
- [ ] Schedule blameless post-mortem with Frank Müller, Isabelle Dupont

## Lessons Learned

We need better load testing to catch these issues before production.
