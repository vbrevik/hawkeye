# Design Doc: Glacier Storage

**Author:** Henrik Larsen
**Date:** 2025-04-23
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using DynamoDB as the primary technology.

### Architecture

```
[Client] → [search-service] → [report-generator] → [Database]
                ↓
          [DynamoDB]
```

The system will consist of:
- **search-service**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **DynamoDB**: Caching and state management

### Data Flow

1. Client sends request to search-service
2. search-service validates the token using Terraform
3. Request is forwarded to report-generator
4. Response is cached in DynamoDB with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| DynamoDB | Fast, well-supported | Higher operational complexity |
| Terraform | Simpler | Less performant |

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
