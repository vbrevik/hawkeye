# Design Doc: Pulse Monitoring

**Author:** Mohamed Al-Rashid
**Date:** 2024-04-25
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using TypeScript as the primary technology.

### Architecture

```
[Client] → [user-service] → [data-warehouse] → [Database]
                ↓
          [TypeScript]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **TypeScript**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using React
3. Request is forwarded to data-warehouse
4. Response is cached in TypeScript with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| TypeScript | Fast, well-supported | Higher operational complexity |
| React | Simpler | Less performant |

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
