# Runbook: search-service

**Last updated:** 2024-05-12
**Owner:** Priya Patel
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for search-service.
The service runs on Helm and uses Nginx for storage.

## Health Check

```bash
curl -s https://internal.example.com/search-service/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `search-service-overview`
2. Inspect slow query logs: `kubectl logs -l app=search-service | grep "slow_query"`
3. Check downstream dependencies: scheduler
4. If needed, scale up: `kubectl scale deployment search-service --replicas=6`

### Issue 2: Goroutine leak in the websocket handler

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/search-service`
2. Rollback if needed: `kubectl rollout undo deployment/search-service`
3. Page Gina Torres if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=search-service`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: David Park
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=search-service --tail=100 -f

# Describe pods
kubectl describe pods -l app=search-service

# Restart deployment
kubectl rollout restart deployment/search-service
```
