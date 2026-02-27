# Design Doc: Meridian Data

**Author:** Alice Chen
**Date:** 2023-10-07
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using gRPC as the primary technology.

### Architecture

```
[Client] → [scheduler] → [audit-logger] → [Database]
                ↓
          [gRPC]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **gRPC**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using DynamoDB
3. Request is forwarded to audit-logger
4. Response is cached in gRPC with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| gRPC | Fast, well-supported | Higher operational complexity |
| DynamoDB | Simpler | Less performant |

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
