# Design Doc: Search Rewrite

**Author:** Kofi Mensah
**Date:** 2025-06-10
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Helm as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [audit-logger] → [Database]
                ↓
          [Helm]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **Helm**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using DynamoDB
3. Request is forwarded to audit-logger
4. Response is cached in Helm with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Helm | Fast, well-supported | Higher operational complexity |
| DynamoDB | Simpler | Less performant |

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
