# Design Doc: Meridian Data

**Author:** Frank Müller
**Date:** 2023-11-18
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to set up alerting for P99 latency
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will migrate the legacy monolith to microservices using React as the primary technology.

### Architecture

```
[Client] → [user-service] → [auth-service] → [Database]
                ↓
          [React]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **auth-service**: Core business logic
- **React**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using Elasticsearch
3. Request is forwarded to auth-service
4. Response is cached in React with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| React | Fast, well-supported | Higher operational complexity |
| Elasticsearch | Simpler | Less performant |

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
