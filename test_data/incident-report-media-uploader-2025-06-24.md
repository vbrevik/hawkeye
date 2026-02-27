# Incident Report — media-uploader — 2025-06-24

**Date:** 2025-06-24
**Severity:** P3
**Duration:** ~41 minutes
**Service:** media-uploader
**Responders:** Gina Torres, Bob Martins

## Summary

Media-uploader experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
41 minutes and affected 31% of traffic.

## Timeline

- **2025-06-24 09:12** — Alerts triggered on Kubernetes metrics
- **2025-06-24 09:18** — Gina Torres acknowledged the alert
- **2025-06-24 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-06-24 09:41** — Mitigation applied (rolled back last deployment)
- **2025-06-24 09:53** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
add rate limiting to the public API. The change was not caught in staging because the
load pattern was different.

## Impact

- 11,482 requests failed
- 304 users affected
- Downstream services impacted: cache-layer, webhook-handler

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kubernetes
- [ ] Schedule blameless post-mortem with Gina Torres, Bob Martins

## Lessons Learned

We need better canary deployments to catch these issues before production.
