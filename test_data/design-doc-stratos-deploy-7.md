# Design Doc: Stratos Deploy

**Author:** Laura Bianchi
**Date:** 2023-10-19
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using DynamoDB as the primary technology.

### Architecture

```
[Client] → [notification-service] → [scheduler] → [Database]
                ↓
          [DynamoDB]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **scheduler**: Core business logic
- **DynamoDB**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using Vault
3. Request is forwarded to scheduler
4. Response is cached in DynamoDB with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| DynamoDB | Fast, well-supported | Higher operational complexity |
| Vault | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle memory leak in the worker pool?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
