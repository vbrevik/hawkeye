# Design Doc: Nexus API

**Author:** Priya Patel
**Date:** 2025-05-02
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to migrate the legacy monolith to microservices
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from token expiry edge case when clock skew > 30s.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using Vault as the primary technology.

### Architecture

```
[Client] → [user-service] → [media-uploader] → [Database]
                ↓
          [Vault]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **media-uploader**: Core business logic
- **Vault**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using Nginx
3. Request is forwarded to media-uploader
4. Response is cached in Vault with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Vault | Fast, well-supported | Higher operational complexity |
| Nginx | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle goroutine leak in the WebSocket handler?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
