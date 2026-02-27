# Design Doc: Lighthouse CMS

**Author:** Jae-won Kim
**Date:** 2023-02-09
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, auth-service suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Rust as the primary technology.

### Architecture

```
[Client] → [payment-processor] → [event-bus] → [Database]
                ↓
          [Rust]
```

The system will consist of:
- **payment-processor**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Rust**: Caching and state management

### Data Flow

1. Client sends request to payment-processor
2. payment-processor validates the token using S3
3. Request is forwarded to event-bus
4. Response is cached in Rust with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Rust | Fast, well-supported | Higher operational complexity |
| S3 | Simpler | Less performant |

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
