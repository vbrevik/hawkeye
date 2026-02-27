# Design Doc: Glacier Storage

**Author:** David Park
**Date:** 2023-07-03
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Go as the primary technology.

### Architecture

```
[Client] → [report-generator] → [auth-service] → [Database]
                ↓
          [Go]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Go**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Axum
3. Request is forwarded to auth-service
4. Response is cached in Go with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Go | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle slow query on the user lookup table (missing index)?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
