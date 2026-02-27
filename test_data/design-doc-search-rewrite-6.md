# Design Doc: Search Rewrite

**Author:** Priya Patel
**Date:** 2024-05-03
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using FastAPI as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [scheduler] → [Database]
                ↓
          [FastAPI]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **scheduler**: Core business logic
- **FastAPI**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using TypeScript
3. Request is forwarded to scheduler
4. Response is cached in FastAPI with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| FastAPI | Fast, well-supported | Higher operational complexity |
| TypeScript | Simpler | Less performant |

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
