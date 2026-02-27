# Design Doc: Migration to K8s

**Author:** Gina Torres
**Date:** 2026-01-23
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using React as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [media-uploader] → [Database]
                ↓
          [React]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **media-uploader**: Core business logic
- **React**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using Kafka
3. Request is forwarded to media-uploader
4. Response is cached in React with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| React | Fast, well-supported | Higher operational complexity |
| Kafka | Simpler | Less performant |

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
