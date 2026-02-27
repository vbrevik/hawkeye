# Research Notes — FastAPI Evaluation

**Author:** David Park
**Date:** 2023-04-27

## Purpose

Evaluating FastAPI for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for cache-layer.

## Key Findings

### Performance

Initial benchmarks show FastAPI handles approximately 55,870 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is similar — roughly
197MB under load compared to our current 539MB.

### Operational Complexity

FastAPI requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The FastAPI community is growing.
Documentation quality is good.
Last major release: 2025-10-21.

### Integration

Integration with our existing report-generator is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Evaluate further FastAPI.

**Rationale:** Decided to go with a pull-based deployment model using ArgoCD.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Mohamed Al-Rashid
- [ ] Prototype integration with event-bus
- [ ] Get sign-off from Henrik Larsen
