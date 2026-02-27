# Runbook: user-service

**Last updated:** 2025-03-06
**Owner:** Bob Martins
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for user-service.
The service runs on gRPC and uses Elasticsearch for storage.

## Health Check

```bash
curl -s https://internal.example.com/user-service/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `user-service-overview`
2. Inspect slow query logs: `kubectl logs -l app=user-service | grep "slow_query"`
3. Check downstream dependencies: search-service
4. If needed, scale up: `kubectl scale deployment user-service --replicas=6`

### Issue 2: Disk i/o bottleneck during bulk import

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/user-service`
2. Rollback if needed: `kubectl rollout undo deployment/user-service`
3. Page Gina Torres if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=user-service`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Kofi Mensah
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=user-service --tail=100 -f

# Describe pods
kubectl describe pods -l app=user-service

# Restart deployment
kubectl rollout restart deployment/user-service
```
