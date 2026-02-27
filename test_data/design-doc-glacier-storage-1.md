# Design Doc: Glacier Storage

**Author:** Kofi Mensah
**Date:** 2024-07-10
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using GraphQL as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [cache-layer] → [Database]
                ↓
          [GraphQL]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **GraphQL**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Redis
3. Request is forwarded to cache-layer
4. Response is cached in GraphQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| GraphQL | Fast, well-supported | Higher operational complexity |
| Redis | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle cache invalidation not propagating across regions?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
