# Design Doc: Beacon Analytics

**Author:** Henrik Larsen
**Date:** 2023-08-31
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using Rust as the primary technology.

### Architecture

```
[Client] → [scheduler] → [analytics-pipeline] → [Database]
                ↓
          [Rust]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **Rust**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using Go
3. Request is forwarded to analytics-pipeline
4. Response is cached in Rust with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Rust | Fast, well-supported | Higher operational complexity |
| Go | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle slow query on the user lookup table (missing index)?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
