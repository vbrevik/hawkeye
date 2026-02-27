# Design Doc: Atlas Platform

**Author:** Frank Müller
**Date:** 2024-12-28
**Status:** Draft

## Overview

This document describes the design for Atlas Platform. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Axum as the primary technology.

### Architecture

```
[Client] → [search-service] → [auth-service] → [Database]
                ↓
          [Axum]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Axum**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using React
3. Request is forwarded to auth-service
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

- How do we handle race condition during concurrent writes?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
