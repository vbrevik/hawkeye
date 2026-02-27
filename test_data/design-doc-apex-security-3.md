# Design Doc: Apex Security

**Author:** Gina Torres
**Date:** 2026-01-05
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, webhook-handler suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using gRPC as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [scheduler] → [Database]
                ↓
          [gRPC]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **scheduler**: Core business logic
- **gRPC**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using Celery
3. Request is forwarded to scheduler
4. Response is cached in gRPC with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| gRPC | Fast, well-supported | Higher operational complexity |
| Celery | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle flaky tests in the integration suite?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
