# Research Notes — Axum Evaluation

**Author:** Bob Martins
**Date:** 2025-07-17

## Purpose

Evaluating Axum for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for api-gateway.

## Key Findings

### Performance

Initial benchmarks show Axum handles approximately 11,875 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is lower — roughly
130MB under load compared to our current 502MB.

### Operational Complexity

Axum requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Axum community is growing.
Documentation quality is adequate.
Last major release: 2025-03-01.

### Integration

Integration with our existing api-gateway is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats Axum.

**Rationale:** Feature flags will be managed via LaunchDarkly.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Sofia Andersen
- [ ] Prototype integration with search-service
- [ ] Get sign-off from Kofi Mensah
