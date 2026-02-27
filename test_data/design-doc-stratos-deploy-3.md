# Design Doc: Stratos Deploy

**Author:** Mohamed Al-Rashid
**Date:** 2025-11-02
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Terraform as the primary technology.

### Architecture

```
[Client] → [notification-service] → [auth-service] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using ArgoCD
3. Request is forwarded to auth-service
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| ArgoCD | Simpler | Less performant |

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
