# Design Doc: Auth Overhaul

**Author:** Ravi Sharma
**Date:** 2026-02-11
**Status:** Draft

## Overview

This document describes the design for Auth Overhaul. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using FastAPI as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [search-service] → [Database]
                ↓
          [FastAPI]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **search-service**: Core business logic
- **FastAPI**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using PostgreSQL
3. Request is forwarded to search-service
4. Response is cached in FastAPI with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| FastAPI | Fast, well-supported | Higher operational complexity |
| PostgreSQL | Simpler | Less performant |

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
