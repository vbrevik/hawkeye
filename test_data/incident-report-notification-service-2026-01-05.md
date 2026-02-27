# Incident Report — notification-service — 2026-01-05

**Date:** 2026-01-05
**Severity:** P2
**Duration:** ~21 minutes
**Service:** notification-service
**Responders:** Ravi Sharma, Tomas Novak, Quinn Murphy

## Summary

Notification-service experienced an outage due to memory leak in the worker pool. The incident lasted approximately
21 minutes and affected 71% of traffic.

## Timeline

- **2026-01-05 09:12** — Alerts triggered on Elasticsearch metrics
- **2026-01-05 09:18** — Quinn Murphy acknowledged the alert
- **2026-01-05 09:25** — Root cause identified: memory leak in the worker pool
- **2026-01-05 09:41** — Mitigation applied (rolled back last deployment)
- **2026-01-05 09:33** — Service fully restored

## Root Cause

The root cause was memory leak in the worker pool. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 46,876 requests failed
- 420 users affected
- Downstream services impacted: scheduler, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for memory leak in the worker pool
3. Updated runbook for notification-service

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Ravi Sharma, Tomas Novak

## Lessons Learned

We need better staging parity to catch these issues before production.
