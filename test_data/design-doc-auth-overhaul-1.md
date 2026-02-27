# Design Doc: Auth Overhaul

**Author:** Oscar Lindberg
**Date:** 2025-06-27
**Status:** Draft

## Overview

This document describes the design for Auth Overhaul. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using S3 as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [analytics-pipeline] → [Database]
                ↓
          [S3]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **S3**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using gRPC
3. Request is forwarded to analytics-pipeline
4. Response is cached in S3 with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| S3 | Fast, well-supported | Higher operational complexity |
| gRPC | Simpler | Less performant |

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
