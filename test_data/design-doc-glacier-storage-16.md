# Design Doc: Glacier Storage

**Author:** Isabelle Dupont
**Date:** 2023-07-08
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, webhook-handler suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using RabbitMQ as the primary technology.

### Architecture

```
[Client] → [report-generator] → [api-gateway] → [Database]
                ↓
          [RabbitMQ]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **RabbitMQ**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Nginx
3. Request is forwarded to api-gateway
4. Response is cached in RabbitMQ with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| RabbitMQ | Fast, well-supported | Higher operational complexity |
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
