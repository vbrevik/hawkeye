# Runbook: cache-layer

**Last updated:** 2024-06-15
**Owner:** Frank Müller
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for cache-layer.
The service runs on TypeScript and uses RabbitMQ for storage.

## Health Check

```bash
curl -s https://internal.example.com/cache-layer/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `cache-layer-overview`
2. Inspect slow query logs: `kubectl logs -l app=cache-layer | grep "slow_query"`
3. Check downstream dependencies: webhook-handler
4. If needed, scale up: `kubectl scale deployment cache-layer --replicas=6`

### Issue 2: Memory leak in the worker pool

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/cache-layer`
2. Rollback if needed: `kubectl rollout undo deployment/cache-layer`
3. Page Clara Johansson if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=cache-layer`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Nadia Kovač
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=cache-layer --tail=100 -f

# Describe pods
kubectl describe pods -l app=cache-layer

# Restart deployment
kubectl rollout restart deployment/cache-layer
```
