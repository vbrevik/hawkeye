# Design Doc: Stratos Deploy

**Author:** Henrik Larsen
**Date:** 2024-05-20
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Celery as the primary technology.

### Architecture

```
[Client] → [auth-service] → [webhook-handler] → [Database]
                ↓
          [Celery]
```

The system will consist of:
- **auth-service**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **Celery**: Caching and state management

### Data Flow

1. Client sends request to auth-service
2. auth-service validates the token using PostgreSQL
3. Request is forwarded to webhook-handler
4. Response is cached in Celery with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Celery | Fast, well-supported | Higher operational complexity |
| PostgreSQL | Simpler | Less performant |

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
