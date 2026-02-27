# Design Doc: Lighthouse CMS

**Author:** David Park
**Date:** 2025-10-08
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will implement circuit breakers for downstream calls using ArgoCD as the primary technology.

### Architecture

```
[Client] → [user-service] → [event-bus] → [Database]
                ↓
          [ArgoCD]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **ArgoCD**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using FastAPI
3. Request is forwarded to event-bus
4. Response is cached in ArgoCD with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| ArgoCD | Fast, well-supported | Higher operational complexity |
| FastAPI | Simpler | Less performant |

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
