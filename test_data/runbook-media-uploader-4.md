# Runbook: media-uploader

**Last updated:** 2024-09-07
**Owner:** Bob Martins
**Severity scope:** P1, P2

## Overview

This runbook covers common operational procedures for media-uploader.
The service runs on Kubernetes and uses React for storage.

## Health Check

```bash
curl -s https://internal.example.com/media-uploader/health | jq .
```

Expected response:
```json
{"status": "ok", "version": "1.4.2", "uptime": 123456}
```

## Common Issues

### Issue 1: High latency

**Symptoms:** P99 latency > 500ms, alerts firing in Grafana.

**Steps:**
1. Check CPU and memory usage in Grafana dashboard `media-uploader-overview`
2. Inspect slow query logs: `kubectl logs -l app=media-uploader | grep "slow_query"`
3. Check downstream dependencies: cache-layer
4. If needed, scale up: `kubectl scale deployment media-uploader --replicas=6`

### Issue 2: Token expiry edge case when clock skew > 30s

**Symptoms:** Error rate increasing, customer reports.

**Steps:**
1. Check recent deployments: `kubectl rollout history deployment/media-uploader`
2. Rollback if needed: `kubectl rollout undo deployment/media-uploader`
3. Page Ravi Sharma if issue persists after rollback

### Issue 3: Out of memory

**Symptoms:** OOMKilled pods, restarts increasing.

**Steps:**
1. `kubectl top pods -l app=media-uploader`
2. Check for memory leaks: `kubectl exec -it <pod> -- pprof http://localhost:6060/debug/pprof/heap`
3. Increase memory limit in Helm values and redeploy

## Escalation

- Primary on-call: Check PagerDuty
- Secondary: Oscar Lindberg
- Slack: #incident-response

## Useful Commands

```bash
# Get pod logs
kubectl logs -l app=media-uploader --tail=100 -f

# Describe pods
kubectl describe pods -l app=media-uploader

# Restart deployment
kubectl rollout restart deployment/media-uploader
```
