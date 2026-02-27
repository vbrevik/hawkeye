# Design Doc: Migration to K8s

**Author:** Sofia Andersen
**Date:** 2024-07-31
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, webhook-handler suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Kubernetes as the primary technology.

### Architecture

```
[Client] → [report-generator] → [event-bus] → [Database]
                ↓
          [Kubernetes]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Kubernetes**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Nginx
3. Request is forwarded to event-bus
4. Response is cached in Kubernetes with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kubernetes | Fast, well-supported | Higher operational complexity |
| Nginx | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle race condition during concurrent writes?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
