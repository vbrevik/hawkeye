# Design Doc: Atlas Platform

**Author:** Henrik Larsen
**Date:** 2023-07-19
**Status:** Draft

## Overview

This document describes the design for Atlas Platform. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using PostgreSQL as the primary technology.

### Architecture

```
[Client] → [notification-service] → [data-warehouse] → [Database]
                ↓
          [PostgreSQL]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **PostgreSQL**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using Docker
3. Request is forwarded to data-warehouse
4. Response is cached in PostgreSQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| PostgreSQL | Fast, well-supported | Higher operational complexity |
| Docker | Simpler | Less performant |

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
