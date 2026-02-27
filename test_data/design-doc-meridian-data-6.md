# Design Doc: Meridian Data

**Author:** Gina Torres
**Date:** 2023-06-09
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using FastAPI as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [audit-logger] → [Database]
                ↓
          [FastAPI]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **FastAPI**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using Redis
3. Request is forwarded to audit-logger
4. Response is cached in FastAPI with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| FastAPI | Fast, well-supported | Higher operational complexity |
| Redis | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle SSL certificate not renewing automatically?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
