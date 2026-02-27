# Incident Report — search-service — 2025-11-17

**Date:** 2025-11-17
**Severity:** P1
**Duration:** ~113 minutes
**Service:** search-service
**Responders:** Bob Martins, Quinn Murphy

## Summary

Search-service experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
113 minutes and affected 78% of traffic.

## Timeline

- **2025-11-17 09:12** — Alerts triggered on FastAPI metrics
- **2025-11-17 09:18** — Quinn Murphy acknowledged the alert
- **2025-11-17 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-11-17 09:41** — Mitigation applied (rolled back last deployment)
- **2025-11-17 10:65** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 4,262 requests failed
- 138 users affected
- Downstream services impacted: audit-logger, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for search-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for FastAPI
- [ ] Schedule blameless post-mortem with Bob Martins, Quinn Murphy

## Lessons Learned

We need better integration tests to catch these issues before production.
