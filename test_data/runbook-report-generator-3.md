# Runbook: report-generator

**Last updated:** 2024-01-24
**Owner:** Jae-won Kim
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for report-generator.
The service runs on TypeScript and uses Kafka for storage.

## Health Check

```bash
curl -s https://internal.example.com/report-generator/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `report-generator-overview`
2. Inspect slow query logs: `kubectl logs -l app=report-generator | grep "slow_query"`
3. Check downstream dependencies: auth-service
4. If needed, scale up: `kubectl scale deployment report-generator --replicas=6`

### Issue 2: Cache invalidation not propagating across regions

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/report-generator`
2. Rollback if needed: `kubectl rollout undo deployment/report-generator`
3. Page Mohamed Al-Rashid if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=report-generator`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Nadia Kovač
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=report-generator --tail=100 -f

# Describe pods
kubectl describe pods -l app=report-generator

# Restart deployment
kubectl rollout restart deployment/report-generator
```
