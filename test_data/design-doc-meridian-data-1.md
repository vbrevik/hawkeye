# Design Doc: Meridian Data

**Author:** Elena Rossi
**Date:** 2024-01-08
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Rust as the primary technology.

### Architecture

```
[Client] → [cache-layer] → [webhook-handler] → [Database]
                ↓
          [Rust]
```

The system will consist of:
- **cache-layer**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **Rust**: Caching and state management

### Data Flow

1. Client sends request to cache-layer
2. cache-layer validates the token using Axum
3. Request is forwarded to webhook-handler
4. Response is cached in Rust with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Rust | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle cache invalidation not propagating across regions?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
