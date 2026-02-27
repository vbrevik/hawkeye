# Design Doc: Core Refactor Q1

**Author:** Mohamed Al-Rashid
**Date:** 2024-08-02
**Status:** Draft

## Overview

This document describes the design for Core Refactor Q1. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using Elasticsearch as the primary technology.

### Architecture

```
[Client] → [cache-layer] → [media-uploader] → [Database]
                ↓
          [Elasticsearch]
```

The system will consist of:
- **cache-layer**: Handles incoming requests and authentication
- **media-uploader**: Core business logic
- **Elasticsearch**: Caching and state management

### Data Flow

1. Client sends request to cache-layer
2. cache-layer validates the token using React
3. Request is forwarded to media-uploader
4. Response is cached in Elasticsearch with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Elasticsearch | Fast, well-supported | Higher operational complexity |
| React | Simpler | Less performant |

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
