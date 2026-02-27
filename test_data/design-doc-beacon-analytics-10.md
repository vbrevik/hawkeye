# Design Doc: Beacon Analytics

**Author:** Gina Torres
**Date:** 2025-06-02
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using ArgoCD as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [cache-layer] → [Database]
                ↓
          [ArgoCD]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **ArgoCD**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using Redis
3. Request is forwarded to cache-layer
4. Response is cached in ArgoCD with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| ArgoCD | Fast, well-supported | Higher operational complexity |
| Redis | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle flaky tests in the integration suite?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
