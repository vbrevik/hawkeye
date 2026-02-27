# Runbook: webhook-handler

**Last updated:** 2026-01-17
**Owner:** Isabelle Dupont
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for webhook-handler.
The service runs on Axum and uses Celery for storage.

## Health Check

```bash
curl -s https://internal.example.com/webhook-handler/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `webhook-handler-overview`
2. Inspect slow query logs: `kubectl logs -l app=webhook-handler | grep "slow_query"`
3. Check downstream dependencies: auth-service
4. If needed, scale up: `kubectl scale deployment webhook-handler --replicas=6`

### Issue 2: Token expiry edge case when clock skew > 30s

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/webhook-handler`
2. Rollback if needed: `kubectl rollout undo deployment/webhook-handler`
3. Page Jae-won Kim if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=webhook-handler`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Laura Bianchi
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=webhook-handler --tail=100 -f

# Describe pods
kubectl describe pods -l app=webhook-handler

# Restart deployment
kubectl rollout restart deployment/webhook-handler
```
