# Design Doc: Project Phoenix

**Author:** Nadia Kovač
**Date:** 2025-06-02
**Status:** Draft

## Overview

This document describes the design for Project Phoenix. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using S3 as the primary technology.

### Architecture

```
[Client] → [scheduler] → [user-service] → [Database]
                ↓
          [S3]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **S3**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Helm
3. Request is forwarded to user-service
4. Response is cached in S3 with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| S3 | Fast, well-supported | Higher operational complexity |
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
