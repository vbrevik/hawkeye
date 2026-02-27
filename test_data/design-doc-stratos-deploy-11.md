# Design Doc: Stratos Deploy

**Author:** Oscar Lindberg
**Date:** 2024-07-21
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, api-gateway suffers from cache invalidation not propagating across regions.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using Axum as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [report-generator] → [Database]
                ↓
          [Axum]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **Axum**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Redis
3. Request is forwarded to report-generator
4. Response is cached in Axum with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Axum | Fast, well-supported | Higher operational complexity |
| Redis | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle retry storm after upstream timeout?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
