# Design Doc: Stratos Deploy

**Author:** Tomas Novak
**Date:** 2023-06-10
**Status:** Draft

## Overview

This document describes the design for Stratos Deploy. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, cache-layer suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using S3 as the primary technology.

### Architecture

```
[Client] → [webhook-handler] → [media-uploader] → [Database]
                ↓
          [S3]
```

The system will consist of:
- **webhook-handler**: Handles incoming requests and authentication
- **media-uploader**: Core business logic
- **S3**: Caching and state management

### Data Flow

1. Client sends request to webhook-handler
2. webhook-handler validates the token using Terraform
3. Request is forwarded to media-uploader
4. Response is cached in S3 with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| S3 | Fast, well-supported | Higher operational complexity |
| Terraform | Simpler | Less performant |

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
