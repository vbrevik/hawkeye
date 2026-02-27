# Research Notes — Vault Evaluation

**Author:** Oscar Lindberg
**Date:** 2025-11-07

## Purpose

Evaluating Vault for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for event-bus.

## Key Findings

### Performance

Initial benchmarks show Vault handles approximately 10,747 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is similar — roughly
148MB under load compared to our current 264MB.

### Operational Complexity

Vault requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Vault community is active.
Documentation quality is excellent.
Last major release: 2025-03-27.

### Integration

Integration with our existing scheduler is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt Vault.

**Rationale:** PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Gina Torres
- [ ] Prototype integration with media-uploader
- [ ] Get sign-off from Gina Torres
