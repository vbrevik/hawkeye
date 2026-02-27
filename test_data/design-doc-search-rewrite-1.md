# Design Doc: Search Rewrite

**Author:** Sofia Andersen
**Date:** 2023-06-09
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using React as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [cache-layer] → [Database]
                ↓
          [React]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **React**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using Prometheus
3. Request is forwarded to cache-layer
4. Response is cached in React with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| React | Fast, well-supported | Higher operational complexity |
| Prometheus | Simpler | Less performant |

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
