# Runbook: analytics-pipeline

**Last updated:** 2023-07-06
**Owner:** David Park
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for analytics-pipeline.
The service runs on Elasticsearch and uses Terraform for storage.

## Health Check

```bash
curl -s https://internal.example.com/analytics-pipeline/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `analytics-pipeline-overview`
2. Inspect slow query logs: `kubectl logs -l app=analytics-pipeline | grep "slow_query"`
3. Check downstream dependencies: api-gateway
4. If needed, scale up: `kubectl scale deployment analytics-pipeline --replicas=6`

### Issue 2: Ssl certificate not renewing automatically

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/analytics-pipeline`
2. Rollback if needed: `kubectl rollout undo deployment/analytics-pipeline`
3. Page Frank Müller if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=analytics-pipeline`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Jae-won Kim
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=analytics-pipeline --tail=100 -f

# Describe pods
kubectl describe pods -l app=analytics-pipeline

# Restart deployment
kubectl rollout restart deployment/analytics-pipeline
```
