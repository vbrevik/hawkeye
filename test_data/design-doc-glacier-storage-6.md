# Design Doc: Glacier Storage

**Author:** Tomas Novak
**Date:** 2025-03-18
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Elasticsearch as the primary technology.

### Architecture

```
[Client] → [search-service] → [audit-logger] → [Database]
                ↓
          [Elasticsearch]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **Elasticsearch**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Axum
3. Request is forwarded to audit-logger
4. Response is cached in Elasticsearch with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Elasticsearch | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle flaky tests in the integration suite?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
