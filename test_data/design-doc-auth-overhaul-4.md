# Design Doc: Auth Overhaul

**Author:** Frank Müller
**Date:** 2023-04-18
**Status:** Draft

## Overview

This document describes the design for Auth Overhaul. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using PostgreSQL as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [event-bus] → [Database]
                ↓
          [PostgreSQL]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **PostgreSQL**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using DynamoDB
3. Request is forwarded to event-bus
4. Response is cached in PostgreSQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| PostgreSQL | Fast, well-supported | Higher operational complexity |
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
