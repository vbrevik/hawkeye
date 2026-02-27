# Design Doc: Glacier Storage

**Author:** Quinn Murphy
**Date:** 2023-05-03
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Terraform as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [event-bus] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using DynamoDB
3. Request is forwarded to event-bus
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| DynamoDB | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle goroutine leak in the WebSocket handler?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
