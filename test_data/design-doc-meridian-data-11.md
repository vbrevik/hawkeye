# Design Doc: Meridian Data

**Author:** Nadia Kovač
**Date:** 2024-12-09
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using DynamoDB as the primary technology.

### Architecture

```
[Client] → [search-service] → [cache-layer] → [Database]
                ↓
          [DynamoDB]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **DynamoDB**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Grafana
3. Request is forwarded to cache-layer
4. Response is cached in DynamoDB with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| DynamoDB | Fast, well-supported | Higher operational complexity |
| Grafana | Simpler | Less performant |

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
