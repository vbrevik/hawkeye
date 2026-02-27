# Design Doc: Beacon Analytics

**Author:** David Park
**Date:** 2023-10-10
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using Helm as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [event-bus] → [Database]
                ↓
          [Helm]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Helm**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using Grafana
3. Request is forwarded to event-bus
4. Response is cached in Helm with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Helm | Fast, well-supported | Higher operational complexity |
| Grafana | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle goroutine leak in the WebSocket handler?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
