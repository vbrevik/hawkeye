# Research Notes — Axum Evaluation

**Author:** Priya Patel
**Date:** 2023-11-16

## Purpose

Evaluating Axum for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for auth-service.

## Key Findings

### Performance

Initial benchmarks show Axum handles approximately 54,993 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is lower — roughly
438MB under load compared to our current 564MB.

### Operational Complexity

Axum requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Axum community is mature and stable.
Documentation quality is adequate.
Last major release: 2025-03-20.

### Integration

Integration with our existing cache-layer is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Axum.

**Rationale:** Will use Redis for session storage — simple and battle-tested.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Mohamed Al-Rashid
- [ ] Prototype integration with event-bus
- [ ] Get sign-off from Jae-won Kim
