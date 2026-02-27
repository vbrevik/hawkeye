# Design Doc: Meridian Data

**Author:** Nadia Kovač
**Date:** 2023-02-24
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using GraphQL as the primary technology.

### Architecture

```
[Client] → [scheduler] → [notification-service] → [Database]
                ↓
          [GraphQL]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **notification-service**: Core business logic
- **GraphQL**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Grafana
3. Request is forwarded to notification-service
4. Response is cached in GraphQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| GraphQL | Fast, well-supported | Higher operational complexity |
| Grafana | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle disk I/O bottleneck during bulk import?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
