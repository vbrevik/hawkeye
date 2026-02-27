# Design Doc: Nexus API

**Author:** Mohamed Al-Rashid
**Date:** 2025-05-16
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to add structured logging with trace IDs
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using React as the primary technology.

### Architecture

```
[Client] → [event-bus] → [search-service] → [Database]
                ↓
          [React]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **search-service**: Core business logic
- **React**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using Terraform
3. Request is forwarded to search-service
4. Response is cached in React with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| React | Fast, well-supported | Higher operational complexity |
| Terraform | Simpler | Less performant |

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
