# Design Doc: Beacon Analytics

**Author:** Sofia Andersen
**Date:** 2025-03-14
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Vault as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [audit-logger] → [Database]
                ↓
          [Vault]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **Vault**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using DynamoDB
3. Request is forwarded to audit-logger
4. Response is cached in Vault with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Vault | Fast, well-supported | Higher operational complexity |
| DynamoDB | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle disk I/O bottleneck during bulk import?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
