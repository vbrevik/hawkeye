# Design Doc: Atlas Platform

**Author:** Oscar Lindberg
**Date:** 2023-08-06
**Status:** Draft

## Overview

This document describes the design for Atlas Platform. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using ArgoCD as the primary technology.

### Architecture

```
[Client] → [user-service] → [api-gateway] → [Database]
                ↓
          [ArgoCD]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **ArgoCD**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using Axum
3. Request is forwarded to api-gateway
4. Response is cached in ArgoCD with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| ArgoCD | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle memory leak in the worker pool?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
