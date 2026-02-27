# Design Doc: Core Refactor Q1

**Author:** Kofi Mensah
**Date:** 2025-05-15
**Status:** Draft

## Overview

This document describes the design for Core Refactor Q1. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using React as the primary technology.

### Architecture

```
[Client] → [notification-service] → [payment-processor] → [Database]
                ↓
          [React]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **payment-processor**: Core business logic
- **React**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using GraphQL
3. Request is forwarded to payment-processor
4. Response is cached in React with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| React | Fast, well-supported | Higher operational complexity |
| GraphQL | Simpler | Less performant |

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
