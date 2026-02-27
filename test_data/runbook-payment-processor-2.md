# Runbook: payment-processor

**Last updated:** 2024-10-26
**Owner:** Alice Chen
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for payment-processor.
The service runs on DynamoDB and uses gRPC for storage.

## Health Check

```bash
curl -s https://internal.example.com/payment-processor/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `payment-processor-overview`
2. Inspect slow query logs: `kubectl logs -l app=payment-processor | grep "slow_query"`
3. Check downstream dependencies: notification-service
4. If needed, scale up: `kubectl scale deployment payment-processor --replicas=6`

### Issue 2: Token expiry edge case when clock skew > 30s

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/payment-processor`
2. Rollback if needed: `kubectl rollout undo deployment/payment-processor`
3. Page Sofia Andersen if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=payment-processor`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: David Park
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=payment-processor --tail=100 -f

# Describe pods
kubectl describe pods -l app=payment-processor

# Restart deployment
kubectl rollout restart deployment/payment-processor
```
