# Design Doc: Glacier Storage

**Author:** Clara Johansson
**Date:** 2025-05-01
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using Prometheus as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [user-service] → [Database]
                ↓
          [Prometheus]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **Prometheus**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using Helm
3. Request is forwarded to user-service
4. Response is cached in Prometheus with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Prometheus | Fast, well-supported | Higher operational complexity |
| Helm | Simpler | Less performant |

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
