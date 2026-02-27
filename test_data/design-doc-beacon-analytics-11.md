# Design Doc: Beacon Analytics

**Author:** Frank Müller
**Date:** 2023-12-08
**Status:** Draft

## Overview

This document describes the design for Beacon Analytics. The goal is to refactor the authentication middleware
in a way that is scalable, maintainable, and observable.

## Problem Statement

Currently, media-uploader suffers from flaky tests in the integration suite.
This leads to degraded performance and reliability. We need a solution
that handles the current load and scales to 10x.

## Proposed Solution

We will review and rotate all secrets in Vault using Go as the primary technology.

### Architecture

```
[Client] → [audit-logger] → [event-bus] → [Database]
                ↓
          [Go]
```

The system will consist of:
- **audit-logger**: Handles incoming requests and authentication
- **event-bus**: Core business logic
- **Go**: Caching and state management

### Data Flow

1. Client sends request to audit-logger
2. audit-logger validates the token using Prometheus
3. Request is forwarded to event-bus
4. Response is cached in Go with TTL of 5 minutes

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Go | Fast, well-supported | Higher operational complexity |
| Prometheus | Simpler | Less performant |

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
