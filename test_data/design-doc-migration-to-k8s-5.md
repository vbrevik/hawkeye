# Design Doc: Migration to K8s

**Author:** Laura Bianchi
**Date:** 2025-03-06
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using Axum as the primary technology.

### Architecture

```
[Client] → [notification-service] → [payment-processor] → [Database]
                ↓
          [Axum]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **payment-processor**: Core business logic
- **Axum**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using React
3. Request is forwarded to payment-processor
4. Response is cached in Axum with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Axum | Fast, well-supported | Higher operational complexity |
| React | Simpler | Less performant |

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
