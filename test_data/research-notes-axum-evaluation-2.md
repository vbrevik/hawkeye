# Research Notes — Axum Evaluation

**Author:** Ravi Sharma
**Date:** 2024-12-22

## Purpose

Evaluating Axum for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for event-bus.

## Key Findings

### Performance

Initial benchmarks show Axum handles approximately 81,389 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is similar — roughly
413MB under load compared to our current 313MB.

### Operational Complexity

Axum requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Axum community is mature and stable.
Documentation quality is adequate.
Last major release: 2025-05-27.

### Integration

Integration with our existing audit-logger is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt Axum.

**Rationale:** We will use Rust for the new service due to memory safety and performance.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Quinn Murphy
- [ ] Prototype integration with api-gateway
- [ ] Get sign-off from Alice Chen
