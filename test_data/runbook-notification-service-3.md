# Runbook: notification-service

**Last updated:** 2025-04-25
**Owner:** Mohamed Al-Rashid
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for notification-service.
The service runs on Nginx and uses Axum for storage.

## Health Check

```bash
curl -s https://internal.example.com/notification-service/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `notification-service-overview`
2. Inspect slow query logs: `kubectl logs -l app=notification-service | grep "slow_query"`
3. Check downstream dependencies: payment-processor
4. If needed, scale up: `kubectl scale deployment notification-service --replicas=6`

### Issue 2: Flaky tests in the integration suite

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/notification-service`
2. Rollback if needed: `kubectl rollout undo deployment/notification-service`
3. Page Frank Müller if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=notification-service`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Gina Torres
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=notification-service --tail=100 -f

# Describe pods
kubectl describe pods -l app=notification-service

# Restart deployment
kubectl rollout restart deployment/notification-service
```
