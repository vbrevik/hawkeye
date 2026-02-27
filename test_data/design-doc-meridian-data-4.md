# Design Doc: Meridian Data

**Author:** Clara Johansson
**Date:** 2025-07-06
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Elasticsearch as the primary technology.

### Architecture

```
[Client] → [event-bus] → [analytics-pipeline] → [Database]
                ↓
          [Elasticsearch]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **Elasticsearch**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using React
3. Request is forwarded to analytics-pipeline
4. Response is cached in Elasticsearch with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Elasticsearch | Fast, well-supported | Higher operational complexity |
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
