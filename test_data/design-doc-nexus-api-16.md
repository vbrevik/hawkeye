# Design Doc: Nexus API

**Author:** Quinn Murphy
**Date:** 2024-01-14
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using Docker as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [notification-service] → [Database]
                ↓
          [Docker]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **notification-service**: Core business logic
- **Docker**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using FastAPI
3. Request is forwarded to notification-service
4. Response is cached in Docker with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Docker | Fast, well-supported | Higher operational complexity |
| FastAPI | Simpler | Less performant |

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
