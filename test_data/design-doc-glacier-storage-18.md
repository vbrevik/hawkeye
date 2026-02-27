# Design Doc: Glacier Storage

**Author:** Kofi Mensah
**Date:** 2024-03-07
**Status:** Draft

## Overview

This document describes the design for Glacier Storage. The goal is to benchmark the new storage backend
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, report-generator suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will refactor the authentication middleware using Vault as the primary technology.

### Architecture

```
[Client] → [report-generator] → [webhook-handler] → [Database]
                ↓
          [Vault]
```

The system will consist of:
- **report-generator**: Handles incoming requests and authentication
- **webhook-handler**: Core business logic
- **Vault**: Caching and state management

### Data Flow

1. Client sends request to report-generator
2. report-generator validates the token using S3
3. Request is forwarded to webhook-handler
4. Response is cached in Vault with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Vault | Fast, well-supported | Higher operational complexity |
| S3 | Simpler | Less performant |

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
