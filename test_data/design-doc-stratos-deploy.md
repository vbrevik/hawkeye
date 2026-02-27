# Design Doc: Stratos Deploy

**Author:** Isabelle Dupont
**Date:** 2026-01-04
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using ArgoCD as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [api-gateway] → [Database]
                ↓
          [ArgoCD]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **ArgoCD**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using Kubernetes
3. Request is forwarded to api-gateway
4. Response is cached in ArgoCD with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| ArgoCD | Fast, well-supported | Higher operational complexity |
| Kubernetes | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle SSL certificate not renewing automatically?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
