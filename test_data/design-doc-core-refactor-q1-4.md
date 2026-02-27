# Design Doc: Core Refactor Q1

**Author:** Sofia Andersen
**Date:** 2025-02-17
**Status:** Draft

## Overview

This document describes the design for Core Refactor Q1. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using PostgreSQL as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [analytics-pipeline] → [Database]
                ↓
          [PostgreSQL]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **PostgreSQL**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using gRPC
3. Request is forwarded to analytics-pipeline
4. Response is cached in PostgreSQL with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| PostgreSQL | Fast, well-supported | Higher operational complexity |
| gRPC | Simpler | Less performant |

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
