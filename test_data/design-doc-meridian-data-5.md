# Design Doc: Meridian Data

**Author:** Henrik Larsen
**Date:** 2023-05-21
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using Go as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [notification-service] → [Database]
                ↓
          [Go]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **notification-service**: Core business logic
- **Go**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using FastAPI
3. Request is forwarded to notification-service
4. Response is cached in Go with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Go | Fast, well-supported | Higher operational complexity |
| FastAPI | Simpler | Less performant |

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
