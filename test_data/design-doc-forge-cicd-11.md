# Design Doc: Forge CI/CD

**Author:** Elena Rossi
**Date:** 2025-05-28
**Status:** Draft

## Overview

This document describes the design for Forge CI/CD. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using S3 as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [report-generator] → [Database]
                ↓
          [S3]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **S3**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using DynamoDB
3. Request is forwarded to report-generator
4. Response is cached in S3 with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| S3 | Fast, well-supported | Higher operational complexity |
| DynamoDB | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle slow query on the user lookup table (missing index)?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
