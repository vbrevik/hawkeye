# Incident Report — analytics-pipeline — 2023-03-05

**Date:** 2023-03-05
**Severity:** P1
**Duration:** ~157 minutes
**Service:** analytics-pipeline
**Responders:** Sofia Andersen, Mohamed Al-Rashid

## Summary

Analytics-pipeline experienced an outage due to SSL certificate not renewing automatically. The incident lasted approximately
157 minutes and affected 19% of traffic.

## Timeline

- **2023-03-05 09:12** — Alerts triggered on Elasticsearch metrics
- **2023-03-05 09:18** — Mohamed Al-Rashid acknowledged the alert
- **2023-03-05 09:25** — Root cause identified: SSL certificate not renewing automatically
- **2023-03-05 09:41** — Mitigation applied (rolled back last deployment)
- **2023-03-05 11:49** — Service fully restored

## Root Cause

The root cause was SSL certificate not renewing automatically. This was introduced in the latest release when
set up alerting for P99 latency. The change was not caught in staging because the
load pattern was different.

## Impact

- 46,620 requests failed
- 17 users affected
- Downstream services impacted: user-service, data-warehouse

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for SSL certificate not renewing automatical
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for Elasticsearch
- [ ] Schedule blameless post-mortem with Sofia Andersen, Mohamed Al-Rashid

## Lessons Learned

We need better staging parity to catch these issues before production.
