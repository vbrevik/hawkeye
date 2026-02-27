# Design Doc: Search Rewrite

**Author:** Mohamed Al-Rashid
**Date:** 2023-05-19
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Nginx as the primary technology.

### Architecture

```
[Client] → [scheduler] → [payment-processor] → [Database]
                ↓
          [Nginx]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **payment-processor**: Core business logic
- **Nginx**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Kafka
3. Request is forwarded to payment-processor
4. Response is cached in Nginx with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Nginx | Fast, well-supported | Higher operational complexity |
| Kafka | Simpler | Less performant |

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
