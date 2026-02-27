# Design Doc: Project Phoenix

**Author:** Alice Chen
**Date:** 2024-10-25
**Status:** Draft

## Overview

This document describes the design for Project Phoenix. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using Kubernetes as the primary technology.

### Architecture

```
[Client] → [user-service] → [auth-service] → [Database]
                ↓
          [Kubernetes]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Kubernetes**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using Grafana
3. Request is forwarded to auth-service
4. Response is cached in Kubernetes with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kubernetes | Fast, well-supported | Higher operational complexity |
| Grafana | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle race condition during concurrent writes?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
