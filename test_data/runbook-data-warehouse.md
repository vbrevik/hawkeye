# Runbook: data-warehouse

**Last updated:** 2025-04-26
**Owner:** Bob Martins
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for data-warehouse.
The service runs on Vault and uses Docker for storage.

## Health Check

```bash
curl -s https://internal.example.com/data-warehouse/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `data-warehouse-overview`
2. Inspect slow query logs: `kubectl logs -l app=data-warehouse | grep "slow_query"`
3. Check downstream dependencies: webhook-handler
4. If needed, scale up: `kubectl scale deployment data-warehouse --replicas=6`

### Issue 2: Cache invalidation not propagating across regions

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/data-warehouse`
2. Rollback if needed: `kubectl rollout undo deployment/data-warehouse`
3. Page Elena Rossi if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=data-warehouse`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Gina Torres
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=data-warehouse --tail=100 -f

# Describe pods
kubectl describe pods -l app=data-warehouse

# Restart deployment
kubectl rollout restart deployment/data-warehouse
```
