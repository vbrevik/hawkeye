# Runbook: audit-logger

**Last updated:** 2023-03-06
**Owner:** Jae-won Kim
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for audit-logger.
The service runs on Redis and uses Nginx for storage.

## Health Check

```bash
curl -s https://internal.example.com/audit-logger/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `audit-logger-overview`
2. Inspect slow query logs: `kubectl logs -l app=audit-logger | grep "slow_query"`
3. Check downstream dependencies: data-warehouse
4. If needed, scale up: `kubectl scale deployment audit-logger --replicas=6`

### Issue 2: Cache invalidation not propagating across regions

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/audit-logger`
2. Rollback if needed: `kubectl rollout undo deployment/audit-logger`
3. Page Elena Rossi if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=audit-logger`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Nadia Kovač
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=audit-logger --tail=100 -f

# Describe pods
kubectl describe pods -l app=audit-logger

# Restart deployment
kubectl rollout restart deployment/audit-logger
```
