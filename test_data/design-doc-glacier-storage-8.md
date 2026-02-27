# Design Doc: Glacier Storage

**Author:** Kofi Mensah
**Date:** 2024-01-11
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from race condition during concurrent writes.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using S3 as the primary technology.

### Architecture

```
[Client] → [event-bus] → [audit-logger] → [Database]
                ↓
          [S3]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **audit-logger**: Core business logic
- **S3**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using Terraform
3. Request is forwarded to audit-logger
4. Response is cached in S3 with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| S3 | Fast, well-supported | Higher operational complexity |
| Terraform | Simpler | Less performant |

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
