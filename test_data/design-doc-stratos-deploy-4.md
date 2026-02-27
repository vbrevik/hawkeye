# Design Doc: Stratos Deploy

**Author:** Alice Chen
**Date:** 2025-08-08
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, auth-service suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Axum as the primary technology.

### Architecture

```
[Client] → [api-gateway] → [data-warehouse] → [Database]
                ↓
          [Axum]
```

The system will consist of:
- **api-gateway**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **Axum**: Caching and state management

### Data Flow

1. Client sends request to api-gateway
2. api-gateway validates the token using Rust
3. Request is forwarded to data-warehouse
4. Response is cached in Axum with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Axum | Fast, well-supported | Higher operational complexity |
| Rust | Simpler | Less performant |

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
