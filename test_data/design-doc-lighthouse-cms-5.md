# Design Doc: Lighthouse CMS

**Author:** Ravi Sharma
**Date:** 2024-05-30
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Redis as the primary technology.

### Architecture

```
[Client] → [search-service] → [webhook-handler] → [Database]
                ↓
          [Redis]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **Redis**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using PostgreSQL
3. Request is forwarded to webhook-handler
4. Response is cached in Redis with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Redis | Fast, well-supported | Higher operational complexity |
| PostgreSQL | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle goroutine leak in the WebSocket handler?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
