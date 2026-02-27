# Design Doc: Glacier Storage

**Author:** Alice Chen
**Date:** 2024-12-28
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, webhook-handler suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using Go as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [event-bus] → [Database]
                ↓
          [Go]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Go**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using gRPC
3. Request is forwarded to event-bus
4. Response is cached in Go with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Go | Fast, well-supported | Higher operational complexity |
| gRPC | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle flaky tests in the integration suite?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
