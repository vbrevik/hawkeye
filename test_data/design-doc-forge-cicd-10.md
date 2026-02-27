# Design Doc: Forge CI/CD

**Author:** Clara Johansson
**Date:** 2024-05-23
**Status:** Draft

## Overview

This document describes the design for Forge CI/CD. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Kubernetes as the primary technology.

### Architecture

```
[Client] → [notification-service] → [media-uploader] → [Database]
                ↓
          [Kubernetes]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **media-uploader**: Core business logic
- **Kubernetes**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using Kafka
3. Request is forwarded to media-uploader
4. Response is cached in Kubernetes with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Kubernetes | Fast, well-supported | Higher operational complexity |
| Kafka | Simpler | Less performant |

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
