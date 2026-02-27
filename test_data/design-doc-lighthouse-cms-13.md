# Design Doc: Lighthouse CMS

**Author:** Oscar Lindberg
**Date:** 2024-03-15
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from SSL certificate not renewing automatically.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using DynamoDB as the primary technology.

### Architecture

```
[Client] → [user-service] → [cache-layer] → [Database]
                ↓
          [DynamoDB]
```

The system will consist of:
- **user-service**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **DynamoDB**: Caching and state management

### Data Flow

1. Client sends request to user-service
2. user-service validates the token using Helm
3. Request is forwarded to cache-layer
4. Response is cached in DynamoDB with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| DynamoDB | Fast, well-supported | Higher operational complexity |
| Helm | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle retry storm after upstream timeout?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
