# Design Doc: Meridian Data

**Author:** Isabelle Dupont
**Date:** 2023-11-14
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, auth-service suffers from goroutine leak in the WebSocket handler.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will document the deployment process using ArgoCD as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [report-generator] → [Database]
                ↓
          [ArgoCD]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **ArgoCD**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Elasticsearch
3. Request is forwarded to report-generator
4. Response is cached in ArgoCD with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| ArgoCD | Fast, well-supported | Higher operational complexity |
| Elasticsearch | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle memory leak in the worker pool?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
