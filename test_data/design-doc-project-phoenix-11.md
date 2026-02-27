# Design Doc: Project Phoenix

**Author:** Clara Johansson
**Date:** 2024-11-14
**Status:** Draft

## Overview

This document describes the design for Project Phoenix. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using SQLite as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [search-service] → [Database]
                ↓
          [SQLite]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **search-service**: Core business logic
- **SQLite**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using ArgoCD
3. Request is forwarded to search-service
4. Response is cached in SQLite with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| SQLite | Fast, well-supported | Higher operational complexity |
| ArgoCD | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle race condition during concurrent writes?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
