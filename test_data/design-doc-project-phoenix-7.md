# Design Doc: Project Phoenix

**Author:** Tomas Novak
**Date:** 2024-01-30
**Status:** Draft

## Overview

This document describes the design for Project Phoenix. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Vault as the primary technology.

### Architecture

```
[Client] → [cache-layer] → [auth-service] → [Database]
                ↓
          [Vault]
```

The system will consist of:
- **cache-layer**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Vault**: Caching and state management

### Data Flow

1. Client sends request to cache-layer
2. cache-layer validates the token using FastAPI
3. Request is forwarded to auth-service
4. Response is cached in Vault with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Vault | Fast, well-supported | Higher operational complexity |
| FastAPI | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle retry storm after upstream timeout?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
