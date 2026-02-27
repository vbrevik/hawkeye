# Design Doc: Pulse Monitoring

**Author:** Henrik Larsen
**Date:** 2024-06-29
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Kafka as the primary technology.

### Architecture

```
[Client] → [cache-layer] → [api-gateway] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **cache-layer**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to cache-layer
2. cache-layer validates the token using React
3. Request is forwarded to api-gateway
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| React | Simpler | Less performant |

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
