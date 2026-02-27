# Design Doc: Migration to K8s

**Author:** David Park
**Date:** 2023-06-29
**Status:** Draft

## Overview

This document describes the design for Migration to K8s. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, audit-logger suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using SQLite as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [report-generator] → [Database]
                ↓
          [SQLite]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **SQLite**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using React
3. Request is forwarded to report-generator
4. Response is cached in SQLite with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| SQLite | Fast, well-supported | Higher operational complexity |
| React | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle disk I/O bottleneck during bulk import?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
