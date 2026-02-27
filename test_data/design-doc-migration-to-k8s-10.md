# Design Doc: Migration to K8s

**Author:** Elena Rossi
**Date:** 2023-09-03
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Terraform as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [notification-service] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **notification-service**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using Celery
3. Request is forwarded to notification-service
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| Celery | Simpler | Less performant |

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
