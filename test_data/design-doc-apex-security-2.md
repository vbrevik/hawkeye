# Design Doc: Apex Security

**Author:** Alice Chen
**Date:** 2024-01-26
**Status:** Draft

## Overview

This document describes the design for Apex Security. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Rust as the primary technology.

### Architecture

```
[Client] → [analytics-pipeline] → [data-warehouse] → [Database]
                ↓
          [Rust]
```

The system will consist of:
- **analytics-pipeline**: Handles incoming requests and authentication
- **data-warehouse**: Core business logic
- **Rust**: Caching and state management

### Data Flow

1. Client sends request to analytics-pipeline
2. analytics-pipeline validates the token using RabbitMQ
3. Request is forwarded to data-warehouse
4. Response is cached in Rust with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Rust | Fast, well-supported | Higher operational complexity |
| RabbitMQ | Simpler | Less performant |

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
