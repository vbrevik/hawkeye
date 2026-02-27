# Design Doc: Core Refactor Q1

**Author:** Mohamed Al-Rashid
**Date:** 2025-03-03
**Status:** Draft

## Overview

This document describes the design for Core Refactor Q1. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using ArgoCD as the primary technology.

### Architecture

```
[Client] → [notification-service] → [data-warehouse] → [Database]
                ↓
          [ArgoCD]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **ArgoCD**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using Nginx
3. Request is forwarded to data-warehouse
4. Response is cached in ArgoCD with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| ArgoCD | Fast, well-supported | Higher operational complexity |
| Nginx | Simpler | Less performant |

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
