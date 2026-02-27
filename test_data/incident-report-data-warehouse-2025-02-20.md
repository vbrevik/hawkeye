# Incident Report — data-warehouse — 2025-02-20

**Date:** 2025-02-20
**Severity:** P2
**Duration:** ~103 minutes
**Service:** data-warehouse
**Responders:** Tomas Novak, Alice Chen

## Summary

Data-warehouse experienced an outage due to goroutine leak in the WebSocket handler. The incident lasted approximately
103 minutes and affected 64% of traffic.

## Timeline

- **2025-02-20 09:12** — Alerts triggered on Kafka metrics
- **2025-02-20 09:18** — Alice Chen acknowledged the alert
- **2025-02-20 09:25** — Root cause identified: goroutine leak in the WebSocket handler
- **2025-02-20 09:41** — Mitigation applied (rolled back last deployment)
- **2025-02-20 10:55** — Service fully restored

## Root Cause

The root cause was goroutine leak in the WebSocket handler. This was introduced in the latest release when
document the deployment process. The change was not caught in staging because the
load pattern was different.

## Impact

- 40,681 requests failed
- 52 users affected
- Downstream services impacted: analytics-pipeline, media-uploader

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for goroutine leak in the WebSocket handler
3. Updated runbook for data-warehouse

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Kafka
- [ ] Schedule blameless post-mortem with Tomas Novak, Alice Chen

## Lessons Learned

We need better staging parity to catch these issues before production.
