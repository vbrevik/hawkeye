# Design Doc: Pulse Monitoring

**Author:** Clara Johansson
**Date:** 2023-10-26
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, user-service suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Go as the primary technology.

### Architecture

```
[Client] → [data-warehouse] → [user-service] → [Database]
                ↓
          [Go]
```

The system will consist of:
- **data-warehouse**: Handles incoming requests and authentication
- **user-service**: Core business logic
- **Go**: Caching and state management

### Data Flow

1. Client sends request to data-warehouse
2. data-warehouse validates the token using Rust
3. Request is forwarded to user-service
4. Response is cached in Go with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Go | Fast, well-supported | Higher operational complexity |
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
