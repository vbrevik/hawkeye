# Research Notes — Rust Evaluation

**Author:** Bob Martins
**Date:** 2025-05-13

## Purpose

Evaluating Rust for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for audit-logger.

## Key Findings

### Performance

Initial benchmarks show Rust handles approximately 44,998 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is lower — roughly
143MB under load compared to our current 519MB.

### Operational Complexity

Rust requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Rust community is growing.
Documentation quality is excellent.
Last major release: 2025-03-04.

### Integration

Integration with our existing cache-layer is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Evaluate further Rust.

**Rationale:** PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Bob Martins
- [ ] Prototype integration with data-warehouse
- [ ] Get sign-off from Priya Patel
