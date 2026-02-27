# Design Doc: Glacier Storage

**Author:** Alice Chen
**Date:** 2024-01-13
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using FastAPI as the primary technology.

### Architecture

```
[Client] → [auth-service] → [api-gateway] → [Database]
                ↓
          [FastAPI]
```

The system will consist of:
- **auth-service**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **FastAPI**: Caching and state management

### Data Flow

1. Client sends request to auth-service
2. auth-service validates the token using Axum
3. Request is forwarded to api-gateway
4. Response is cached in FastAPI with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| FastAPI | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle token expiry edge case when clock skew > 30s?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
