# Design Doc: Lighthouse CMS

**Author:** Sofia Andersen
**Date:** 2026-01-06
**Status:** Draft

## Overview

This document describes the design for Lighthouse CMS. The goal is to document the deployment process
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, event-bus suffers from slow query on the user lookup table (missing index).
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will set up alerting for P99 latency using SQLite as the primary technology.

### Architecture

```
[Client] → [report-generator] → [event-bus] → [Database]
                ↓
          [SQLite]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **SQLite**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using Vault
3. Request is forwarded to event-bus
4. Response is cached in SQLite with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| SQLite | Fast, well-supported | Higher operational complexity |
| Vault | Simpler | Less performant |

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
