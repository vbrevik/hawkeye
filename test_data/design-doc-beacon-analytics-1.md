# Design Doc: Beacon Analytics

**Author:** Nadia Kovač
**Date:** 2023-03-23
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using Terraform as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [search-service] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **search-service**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using Vault
3. Request is forwarded to search-service
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| Vault | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle memory leak in the worker pool?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
