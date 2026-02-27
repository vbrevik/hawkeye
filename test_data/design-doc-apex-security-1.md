# Design Doc: Apex Security

**Author:** Bob Martins
**Date:** 2025-05-16
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using RabbitMQ as the primary technology.

### Architecture

```
[Client] → [search-service] → [user-service] → [Database]
                ↓
          [RabbitMQ]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **RabbitMQ**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Kafka
3. Request is forwarded to user-service
4. Response is cached in RabbitMQ with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| RabbitMQ | Fast, well-supported | Higher operational complexity |
| Kafka | Simpler | Less performant |

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
