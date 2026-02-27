# Runbook: api-gateway

**Last updated:** 2024-09-28
**Owner:** Kofi Mensah
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for api-gateway.
The service runs on GraphQL and uses Docker for storage.

## Health Check

```bash
curl -s https://internal.example.com/api-gateway/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `api-gateway-overview`
2. Inspect slow query logs: `kubectl logs -l app=api-gateway | grep "slow_query"`
3. Check downstream dependencies: api-gateway
4. If needed, scale up: `kubectl scale deployment api-gateway --replicas=6`

### Issue 2: Ssl certificate not renewing automatically

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/api-gateway`
2. Rollback if needed: `kubectl rollout undo deployment/api-gateway`
3. Page Oscar Lindberg if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=api-gateway`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Alice Chen
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=api-gateway --tail=100 -f

# Describe pods
kubectl describe pods -l app=api-gateway

# Restart deployment
kubectl rollout restart deployment/api-gateway
```
