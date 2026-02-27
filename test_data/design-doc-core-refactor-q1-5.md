# Design Doc: Core Refactor Q1

**Author:** Sofia Andersen
**Date:** 2023-12-13
**Status:** Draft

## Overview

This document describes the design for Core Refactor Q1. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Prometheus as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [cache-layer] → [Database]
                ↓
          [Prometheus]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **Prometheus**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using RabbitMQ
3. Request is forwarded to cache-layer
4. Response is cached in Prometheus with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Prometheus | Fast, well-supported | Higher operational complexity |
| RabbitMQ | Simpler | Less performant |

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
