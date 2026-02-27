# Design Doc: Glacier Storage

**Author:** Frank Müller
**Date:** 2024-11-09
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to add rate limiting to the public API
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, auth-service suffers from memory leak in the worker pool.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using React as the primary technology.

### Architecture

```
[Client] → [event-bus] → [report-generator] → [Database]
                ↓
          [React]
```

The system will consist of:
- **event-bus**: Handles incoming requests and authentication
- **report-generator**: Core business logic
- **React**: Caching and state management

### Data Flow

1. Client sends request to event-bus
2. event-bus validates the token using Go
3. Request is forwarded to report-generator
4. Response is cached in React with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| React | Fast, well-supported | Higher operational complexity |
| Go | Simpler | Less performant |

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
