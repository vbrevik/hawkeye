# Design Doc: Lighthouse CMS

**Author:** Gina Torres
**Date:** 2023-11-09
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using DynamoDB as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [user-service] → [Database]
                ↓
          [DynamoDB]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **DynamoDB**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using RabbitMQ
3. Request is forwarded to user-service
4. Response is cached in DynamoDB with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| DynamoDB | Fast, well-supported | Higher operational complexity |
| RabbitMQ | Simpler | Less performant |

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
