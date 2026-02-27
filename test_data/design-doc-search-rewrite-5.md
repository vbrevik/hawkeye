# Design Doc: Search Rewrite

**Author:** Isabelle Dupont
**Date:** 2023-12-26
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using RabbitMQ as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [event-bus] → [Database]
                ↓
          [RabbitMQ]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **RabbitMQ**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using GraphQL
3. Request is forwarded to event-bus
4. Response is cached in RabbitMQ with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| RabbitMQ | Fast, well-supported | Higher operational complexity |
| GraphQL | Simpler | Less performant |

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
