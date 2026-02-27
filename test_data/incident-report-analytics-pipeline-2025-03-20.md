# Incident Report — analytics-pipeline — 2025-03-20

**Date:** 2025-03-20
**Severity:** P3
**Duration:** ~166 minutes
**Service:** analytics-pipeline
**Responders:** Isabelle Dupont, Clara Johansson, Frank Müller, Alice Chen

## Summary

Analytics-pipeline experienced an outage due to flaky tests in the integration suite. The incident lasted approximately
166 minutes and affected 95% of traffic.

## Timeline

- **2025-03-20 09:12** — Alerts triggered on ArgoCD metrics
- **2025-03-20 09:18** — Frank Müller acknowledged the alert
- **2025-03-20 09:25** — Root cause identified: flaky tests in the integration suite
- **2025-03-20 09:41** — Mitigation applied (rolled back last deployment)
- **2025-03-20 11:58** — Service fully restored

## Root Cause

The root cause was flaky tests in the integration suite. This was introduced in the latest release when
benchmark the new storage backend. The change was not caught in staging because the
load pattern was different.

## Impact

- 42,381 requests failed
- 64 users affected
- Downstream services impacted: media-uploader, cache-layer

## Remediation

1. Rolled back the problematic deployment
2. Added monitoring for flaky tests in the integration suite
3. Updated runbook for analytics-pipeline

## Follow-up Actions

- [ ] Add automated test covering this failure mode
- [ ] Review alerting thresholds for ArgoCD
- [ ] Schedule blameless post-mortem with Isabelle Dupont, Clara Johansson

## Lessons Learned

We need better staging parity to catch these issues before production.
