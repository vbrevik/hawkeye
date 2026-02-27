# Design Doc: Beacon Analytics

**Author:** Oscar Lindberg
**Date:** 2024-03-22
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, search-service suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using gRPC as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [cache-layer] → [Database]
                ↓
          [gRPC]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **cache-layer**: Core business logic
- **gRPC**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using Elasticsearch
3. Request is forwarded to cache-layer
4. Response is cached in gRPC with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| gRPC | Fast, well-supported | Higher operational complexity |
| Elasticsearch | Simpler | Less performant |

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
