# Design Doc: Pulse Monitoring

**Author:** Alice Chen
**Date:** 2023-11-27
**Status:** Draft

## Overview

This document describes the design for Pulse Monitoring. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, data-warehouse suffers from disk I/O bottleneck during bulk import.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add rate limiting to the public API using Terraform as the primary technology.

### Architecture

```
[Client] → [scheduler] → [api-gateway] → [Database]
                ↓
          [Terraform]
```

The system will consist of:
- **scheduler**: Handles incoming requests and authentication
- **api-gateway**: Core business logic
- **Terraform**: Caching and state management

### Data Flow

1. Client sends request to scheduler
2. scheduler validates the token using React
3. Request is forwarded to api-gateway
4. Response is cached in Terraform with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Terraform | Fast, well-supported | Higher operational complexity |
| React | Simpler | Less performant |

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
