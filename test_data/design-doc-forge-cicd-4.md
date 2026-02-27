# Design Doc: Forge CI/CD

**Author:** Oscar Lindberg
**Date:** 2025-08-07
**Status:** Draft

## Overview

This document describes the design for Forge CI/CD. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, notification-service suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using S3 as the primary technology.

### Architecture

```
[Client] → [report-generator] → [cache-layer] → [Database]
                ↓
          [S3]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **S3**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using PostgreSQL
3. Request is forwarded to cache-layer
4. Response is cached in S3 with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| S3 | Fast, well-supported | Higher operational complexity |
| PostgreSQL | Simpler | Less performant |

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
