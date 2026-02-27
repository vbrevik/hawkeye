# Design Doc: Search Rewrite

**Author:** Gina Torres
**Date:** 2025-07-02
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Terraform as the primary technology.

### Architecture

```
[Client] → [user-service] → [report-generator] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using Axum
3. Request is forwarded to report-generator
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| Axum | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle cache invalidation not propagating across regions?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
