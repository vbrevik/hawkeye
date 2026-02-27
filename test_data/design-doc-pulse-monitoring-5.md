# Design Doc: Pulse Monitoring

**Author:** Alice Chen
**Date:** 2023-02-11
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using Terraform as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [notification-service] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **notification-service**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using DynamoDB
3. Request is forwarded to notification-service
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

- How do we handle token expiry edge case when clock skew > 30s?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
