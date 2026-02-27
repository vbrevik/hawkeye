# Design Doc: Nexus API

**Author:** Quinn Murphy
**Date:** 2023-07-20
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, payment-processor suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using ArgoCD as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [event-bus] → [Database]
                ↓
          [ArgoCD]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **ArgoCD**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using Kubernetes
3. Request is forwarded to event-bus
4. Response is cached in ArgoCD with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| ArgoCD | Fast, well-supported | Higher operational complexity |
| Kubernetes | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle slow query on the user lookup table (missing index)?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
