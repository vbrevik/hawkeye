# Design Doc: Atlas Platform

**Author:** Clara Johansson
**Date:** 2025-07-13
**Status:** Draft

## Overview

This document describes the design for Atlas Platform. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using SQLite as the primary technology.

### Architecture

```
[Client] → [user-service] → [auth-service] → [Database]
                ↓
          [SQLite]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **SQLite**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using Helm
3. Request is forwarded to auth-service
4. Response is cached in SQLite with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| SQLite | Fast, well-supported | Higher operational complexity |
| Helm | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle retry storm after upstream timeout?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
