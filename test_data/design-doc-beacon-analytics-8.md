# Design Doc: Beacon Analytics

**Author:** Tomas Novak
**Date:** 2024-05-29
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using RabbitMQ as the primary technology.

### Architecture

```
[Client] → [event-bus] → [media-uploader] → [Database]
                ↓
          [RabbitMQ]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **media-uploader**: Core business logic
- **RabbitMQ**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using ArgoCD
3. Request is forwarded to media-uploader
4. Response is cached in RabbitMQ with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| RabbitMQ | Fast, well-supported | Higher operational complexity |
| ArgoCD | Simpler | Less performant |

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
