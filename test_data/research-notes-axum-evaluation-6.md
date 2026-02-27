# Research Notes — Axum Evaluation

**Author:** David Park
**Date:** 2024-04-29

## Purpose

Evaluating Axum for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for audit-logger.

## Key Findings

### Performance

Initial benchmarks show Axum handles approximately 27,806 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is higher — roughly
436MB under load compared to our current 598MB.

### Operational Complexity

Axum requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Axum community is active.
Documentation quality is excellent.
Last major release: 2025-11-10.

### Integration

Integration with our existing user-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt Axum.

**Rationale:** We will use Rust for the new service due to memory safety and performance.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Clara Johansson
- [ ] Prototype integration with event-bus
- [ ] Get sign-off from Laura Bianchi
