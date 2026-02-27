# Design Doc: Nexus API

**Author:** Ravi Sharma
**Date:** 2025-07-19
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Kubernetes as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [user-service] → [Database]
                ↓
          [Kubernetes]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **Kubernetes**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using Elasticsearch
3. Request is forwarded to user-service
4. Response is cached in Kubernetes with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kubernetes | Fast, well-supported | Higher operational complexity |
| Elasticsearch | Simpler | Less performant |

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
