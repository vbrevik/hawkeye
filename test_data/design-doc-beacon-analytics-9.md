# Design Doc: Beacon Analytics

**Author:** Gina Torres
**Date:** 2024-07-05
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Celery as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [search-service] → [Database]
                ↓
          [Celery]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **search-service**: Core business logic
- **Celery**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using Rust
3. Request is forwarded to search-service
4. Response is cached in Celery with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Celery | Fast, well-supported | Higher operational complexity |
| Rust | Simpler | Less performant |

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
