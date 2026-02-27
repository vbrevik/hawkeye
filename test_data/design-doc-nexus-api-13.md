# Design Doc: Nexus API

**Author:** Oscar Lindberg
**Date:** 2023-01-13
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using PostgreSQL as the primary technology.

### Architecture

```
[Client] → [event-bus] → [webhook-handler] → [Database]
                ↓
          [PostgreSQL]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **PostgreSQL**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using Axum
3. Request is forwarded to webhook-handler
4. Response is cached in PostgreSQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| PostgreSQL | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

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
