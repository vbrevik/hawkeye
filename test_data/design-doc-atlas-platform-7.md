# Design Doc: Atlas Platform

**Author:** Kofi Mensah
**Date:** 2025-06-29
**Status:** Draft

## Overview

This document describes the design for Atlas Platform. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using Celery as the primary technology.

### Architecture

```
[Client] → [report-generator] → [user-service] → [Database]
                ↓
          [Celery]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **Celery**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Nginx
3. Request is forwarded to user-service
4. Response is cached in Celery with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Celery | Fast, well-supported | Higher operational complexity |
| Nginx | Simpler | Less performant |

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
