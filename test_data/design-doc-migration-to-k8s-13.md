# Design Doc: Migration to K8s

**Author:** Alice Chen
**Date:** 2025-03-09
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Kubernetes as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [analytics-pipeline] → [Database]
                ↓
          [Kubernetes]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **Kubernetes**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using Elasticsearch
3. Request is forwarded to analytics-pipeline
4. Response is cached in Kubernetes with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kubernetes | Fast, well-supported | Higher operational complexity |
| Elasticsearch | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle disk I/O bottleneck during bulk import?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
