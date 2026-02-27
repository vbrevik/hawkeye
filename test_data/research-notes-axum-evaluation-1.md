# Research Notes — Axum Evaluation

**Author:** Jae-won Kim
**Date:** 2024-05-21

## Purpose

Evaluating Axum for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for webhook-handler.

## Key Findings

### Performance

Initial benchmarks show Axum handles approximately 9,224 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is higher — roughly
228MB under load compared to our current 541MB.

### Operational Complexity

Axum requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Axum community is growing.
Documentation quality is excellent.
Last major release: 2026-02-02.

### Integration

Integration with our existing scheduler is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Axum.

**Rationale:** Agreed to sunset the legacy Python service by end of Q2.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Sofia Andersen
- [ ] Prototype integration with report-generator
- [ ] Get sign-off from Laura Bianchi
