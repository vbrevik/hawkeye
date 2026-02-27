# Design Doc: Project Phoenix

**Author:** Kofi Mensah
**Date:** 2025-12-06
**Status:** Draft

## Overview

This document describes the design for Project Phoenix. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using gRPC as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [media-uploader] → [Database]
                ↓
          [gRPC]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **media-uploader**: Core business logic
- **gRPC**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using RabbitMQ
3. Request is forwarded to media-uploader
4. Response is cached in gRPC with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| gRPC | Fast, well-supported | Higher operational complexity |
| RabbitMQ | Simpler | Less performant |

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
