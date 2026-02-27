# Design Doc: Apex Security

**Author:** Quinn Murphy
**Date:** 2025-06-28
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Kafka as the primary technology.

### Architecture

```
[Client] → [scheduler] → [auth-service] → [Database]
                ↓
          [Kafka]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Kafka**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Nginx
3. Request is forwarded to auth-service
4. Response is cached in Kafka with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kafka | Fast, well-supported | Higher operational complexity |
| Nginx | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle SSL certificate not renewing automatically?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
