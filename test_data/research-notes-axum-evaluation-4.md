# Research Notes — Axum Evaluation

**Author:** Oscar Lindberg
**Date:** 2025-08-17

## Purpose

Evaluating Axum for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for analytics-pipeline.

## Key Findings

### Performance

Initial benchmarks show Axum handles approximately 16,255 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is lower — roughly
269MB under load compared to our current 494MB.

### Operational Complexity

Axum requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Axum community is growing.
Documentation quality is excellent.
Last major release: 2025-08-13.

### Integration

Integration with our existing event-bus is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt Axum.

**Rationale:** Chose gRPC over REST for the internal service mesh.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Elena Rossi
- [ ] Prototype integration with search-service
- [ ] Get sign-off from Elena Rossi
