# Research Notes — Vault Evaluation

**Author:** Isabelle Dupont
**Date:** 2023-10-30

## Purpose

Evaluating Vault for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for api-gateway.

## Key Findings

### Performance

Initial benchmarks show Vault handles approximately 27,621 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is lower — roughly
191MB under load compared to our current 411MB.

### Operational Complexity

Vault requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Vault community is mature and stable.
Documentation quality is adequate.
Last major release: 2025-07-29.

### Integration

Integration with our existing notification-service is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt Vault.

**Rationale:** Will use Redis for session storage — simple and battle-tested.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Isabelle Dupont
- [ ] Prototype integration with cache-layer
- [ ] Get sign-off from Laura Bianchi
