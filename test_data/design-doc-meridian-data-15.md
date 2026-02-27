# Design Doc: Meridian Data

**Author:** Nadia Kovač
**Date:** 2024-12-23
**Status:** Draft

## Overview

This document describes the design for Meridian Data. The goal is to review and rotate all secrets in Vault
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will add structured logging with trace IDs using React as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [event-bus] → [Database]
                ↓
          [React]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **React**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Helm
3. Request is forwarded to event-bus
4. Response is cached in React with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| React | Fast, well-supported | Higher operational complexity |
| Helm | Simpler | Less performant |

## Success Metrics

- P99 latency < 100ms
- Error rate < 0.1%
- Throughput > 10,000 req/s

## Open Questions

- How do we handle cache invalidation not propagating across regions?
- Who owns the on-call rotation for this service?
- What is the rollback strategy if the migration fails?

## Timeline

- Week 1-2: Implementation of core components
- Week 3: Integration testing
- Week 4: Staged rollout
