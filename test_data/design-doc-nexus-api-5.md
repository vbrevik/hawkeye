# Design Doc: Nexus API

**Author:** Mohamed Al-Rashid
**Date:** 2025-05-29
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using SQLite as the primary technology.

### Architecture

```
[Client] → [event-bus] → [data-warehouse] → [Database]
                ↓
          [SQLite]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **SQLite**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using Axum
3. Request is forwarded to data-warehouse
4. Response is cached in SQLite with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| SQLite | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle memory leak in the worker pool?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
