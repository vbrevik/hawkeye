# Design Doc: Search Rewrite

**Author:** Tomas Novak
**Date:** 2023-11-14
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, scheduler suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Nginx as the primary technology.

### Architecture

```
[Client] → [event-bus] → [media-uploader] → [Database]
                ↓
          [Nginx]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **media-uploader**: Core business logic
- **Nginx**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using TypeScript
3. Request is forwarded to media-uploader
4. Response is cached in Nginx with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Nginx | Fast, well-supported | Higher operational complexity |
| TypeScript | Simpler | Less performant |

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
