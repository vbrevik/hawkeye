# Design Doc: Nexus API

**Author:** Jae-won Kim
**Date:** 2023-07-05
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, webhook-handler suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Helm as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [auth-service] → [Database]
                ↓
          [Helm]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Helm**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using ArgoCD
3. Request is forwarded to auth-service
4. Response is cached in Helm with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Helm | Fast, well-supported | Higher operational complexity |
| ArgoCD | Simpler | Less performant |

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
