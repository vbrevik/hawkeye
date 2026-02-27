# Design Doc: Stratos Deploy

**Author:** Jae-won Kim
**Date:** 2024-05-13
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using SQLite as the primary technology.

### Architecture

```
[Client] → [search-service] → [analytics-pipeline] → [Database]
                ↓
          [SQLite]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **SQLite**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Kubernetes
3. Request is forwarded to analytics-pipeline
4. Response is cached in SQLite with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| SQLite | Fast, well-supported | Higher operational complexity |
| Kubernetes | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle retry storm after upstream timeout?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
