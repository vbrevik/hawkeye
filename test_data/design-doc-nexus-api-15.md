# Design Doc: Nexus API

**Author:** Laura Bianchi
**Date:** 2023-06-24
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using Kubernetes as the primary technology.

### Architecture

```
[Client] → [notification-service] → [audit-logger] → [Database]
                ↓
          [Kubernetes]
```

The system will consist of:
- **notification-service**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **Kubernetes**: Caching and state management

### Data Flow

1. Client sends request to notification-service
2. notification-service validates the token using Nginx
3. Request is forwarded to audit-logger
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
