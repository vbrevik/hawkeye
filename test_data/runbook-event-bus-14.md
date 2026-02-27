# Runbook: event-bus

**Last updated:** 2024-09-06
**Owner:** Alice Chen
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for event-bus.
The service runs on Prometheus and uses Rust for storage.

## Health Check

```bash
curl -s https://internal.example.com/event-bus/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `event-bus-overview`
2. Inspect slow query logs: `kubectl logs -l app=event-bus | grep "slow_query"`
3. Check downstream dependencies: api-gateway
4. If needed, scale up: `kubectl scale deployment event-bus --replicas=6`

### Issue 2: Token expiry edge case when clock skew > 30s

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/event-bus`
2. Rollback if needed: `kubectl rollout undo deployment/event-bus`
3. Page Ravi Sharma if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=event-bus`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Jae-won Kim
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=event-bus --tail=100 -f

# Describe pods
kubectl describe pods -l app=event-bus

# Restart deployment
kubectl rollout restart deployment/event-bus
```
