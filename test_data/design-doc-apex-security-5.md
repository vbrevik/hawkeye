# Design Doc: Apex Security

**Author:** Gina Torres
**Date:** 2023-12-09
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Kafka as the primary technology.

### Architecture

```
[Client] → [notification-service] → [auth-service] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using Axum
3. Request is forwarded to auth-service
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

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
