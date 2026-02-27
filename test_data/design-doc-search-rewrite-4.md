# Design Doc: Search Rewrite

**Author:** Sofia Andersen
**Date:** 2024-02-12
**Status:** Draft

## Overview

This document describes the design for Search Rewrite. The goal is to write runbooks for the on-call team
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, analytics-pipeline suffers from retry storm after upstream timeout.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will benchmark the new storage backend using Grafana as the primary technology.

### Architecture

```
[Client] → [media-uploader] → [audit-logger] → [Database]
                ↓
          [Grafana]
```

The system will consist of:
- **media-uploader**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **Grafana**: Caching and state management

### Data Flow

1. Client sends request to media-uploader
2. media-uploader validates the token using Kubernetes
3. Request is forwarded to audit-logger
4. Response is cached in Grafana with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Grafana | Fast, well-supported | Higher operational complexity |
| Kubernetes | Simpler | Less performant |

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
