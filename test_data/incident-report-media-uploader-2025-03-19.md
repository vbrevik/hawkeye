# Incident Report — media-uploader — 2025-03-19

**Date:** 2025-03-19
**Severity:** P2
**Duration:** ~48 minutes
**Service:** media-uploader
**Responders:** Sofia Andersen, Frank Müller

## Summary

Media-uploader experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
48 minutes and affected 69% of traffic.

## Timeline

- **2025-03-19 09:12** — Alerts triggered on Grafana metrics
- **2025-03-19 09:18** — Frank Müller acknowledged the alert
- **2025-03-19 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2025-03-19 09:41** — Mitigation applied (rolled back last deployment)
- **2025-03-19 09:60** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 30,776 requests failed
- 68 users affected
- Downstream services impacted: api-gateway, analytics-pipeline

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for media-uploader

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Grafana
- [ ] Schedule blameless post-mortem with Sofia Andersen, Frank Müller

## Lessons Learned

We need better alerting coverage to catch these issues before production.
