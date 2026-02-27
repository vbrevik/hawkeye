# Design Doc: Search Rewrite

**Author:** Sofia Andersen
**Date:** 2023-07-13
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using Kubernetes as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [event-bus] → [Database]
                ↓
          [Kubernetes]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Kubernetes**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using Helm
3. Request is forwarded to event-bus
4. Response is cached in Kubernetes with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kubernetes | Fast, well-supported | Higher operational complexity |
| Helm | Simpler | Less performant |

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
