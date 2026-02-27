# Design Doc: Glacier Storage

**Author:** Ravi Sharma
**Date:** 2023-03-28
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using Elasticsearch as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [user-service] → [Database]
                ↓
          [Elasticsearch]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **Elasticsearch**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using RabbitMQ
3. Request is forwarded to user-service
4. Response is cached in Elasticsearch with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Elasticsearch | Fast, well-supported | Higher operational complexity |
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
