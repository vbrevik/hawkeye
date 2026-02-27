# Design Doc: Migration to K8s

**Author:** Nadia Kovač
**Date:** 2025-10-30
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using RabbitMQ as the primary technology.

### Architecture

```
[Client] → [event-bus] → [notification-service] → [Database]
                ↓
          [RabbitMQ]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **notification-service**: Core business logic
- **RabbitMQ**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using Vault
3. Request is forwarded to notification-service
4. Response is cached in RabbitMQ with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| RabbitMQ | Fast, well-supported | Higher operational complexity |
| Vault | Simpler | Less performant |

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
