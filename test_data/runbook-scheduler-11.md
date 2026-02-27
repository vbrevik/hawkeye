# Runbook: scheduler

**Last updated:** 2024-10-18
**Owner:** Jae-won Kim
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for scheduler.
The service runs on RabbitMQ and uses TypeScript for storage.

## Health Check

```bash
curl -s https://internal.example.com/scheduler/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `scheduler-overview`
2. Inspect slow query logs: `kubectl logs -l app=scheduler | grep "slow_query"`
3. Check downstream dependencies: report-generator
4. If needed, scale up: `kubectl scale deployment scheduler --replicas=6`

### Issue 2: Race condition during concurrent writes

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/scheduler`
2. Rollback if needed: `kubectl rollout undo deployment/scheduler`
3. Page Laura Bianchi if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=scheduler`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Tomas Novak
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=scheduler --tail=100 -f

# Describe pods
kubectl describe pods -l app=scheduler

# Restart deployment
kubectl rollout restart deployment/scheduler
```
