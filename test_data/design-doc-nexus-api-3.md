# Design Doc: Nexus API

**Author:** Bob Martins
**Date:** 2025-04-02
**Status:** Draft

## Overview

This document describes the design for Nexus API. The goal is to implement circuit breakers for downstream calls
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, auth-service suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will write runbooks for the on-call team using Elasticsearch as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [analytics-pipeline] → [Database]
                ↓
          [Elasticsearch]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **analytics-pipeline**: Core business logic
- **Elasticsearch**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Rust
3. Request is forwarded to analytics-pipeline
4. Response is cached in Elasticsearch with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Elasticsearch | Fast, well-supported | Higher operational complexity |
| Rust | Simpler | Less performant |

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
