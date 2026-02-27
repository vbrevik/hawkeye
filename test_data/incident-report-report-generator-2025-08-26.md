# Incident Report — report-generator — 2025-08-26

**Date:** 2025-08-26
**Severity:** P1
**Duration:** ~92 minutes
**Service:** report-generator
**Responders:** David Park, Henrik Larsen

## Summary

Report-generator experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
92 minutes and affected 50% of traffic.

## Timeline

- **2025-08-26 09:12** — Alerts triggered on RabbitMQ metrics
- **2025-08-26 09:18** — Henrik Larsen acknowledged the alert
- **2025-08-26 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-08-26 09:41** — Mitigation applied (rolled back last deployment)
- **2025-08-26 10:44** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 34,308 requests failed
- 192 users affected
- Downstream services impacted: report-generator, api-gateway

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for report-generator

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for RabbitMQ
- [ ] Schedule blameless post-mortem with David Park, Henrik Larsen

## Lessons Learned

We need better staging parity to catch these issues before production.
