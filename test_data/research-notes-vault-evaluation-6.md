# Research Notes — Vault Evaluation

**Author:** Bob Martins
**Date:** 2024-04-21

## Purpose

Evaluating Vault for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for scheduler.

## Key Findings

### Performance

Initial benchmarks show Vault handles approximately 69,673 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is higher — roughly
209MB under load compared to our current 479MB.

### Operational Complexity

Vault requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Vault community is active.
Documentation quality is excellent.
Last major release: 2025-06-14.

### Integration

Integration with our existing report-generator is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt Vault.

**Rationale:** We will use Rust for the new service due to memory safety and performance.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Isabelle Dupont
- [ ] Prototype integration with report-generator
- [ ] Get sign-off from Gina Torres
