# Design Doc: Glacier Storage

**Author:** Henrik Larsen
**Date:** 2026-02-14
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, webhook-handler suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using GraphQL as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [webhook-handler] → [Database]
                ↓
          [GraphQL]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **GraphQL**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using Elasticsearch
3. Request is forwarded to webhook-handler
4. Response is cached in GraphQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| GraphQL | Fast, well-supported | Higher operational complexity |
| Elasticsearch | Simpler | Less performant |

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
