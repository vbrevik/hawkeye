# Research Notes — Terraform Evaluation

**Author:** Clara Johansson
**Date:** 2023-11-28

## Purpose

Evaluating Terraform for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for user-service.

## Key Findings

### Performance

Initial benchmarks show Terraform handles approximately 13,177 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is higher — roughly
220MB under load compared to our current 130MB.

### Operational Complexity

Terraform requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Terraform community is growing.
Documentation quality is excellent.
Last major release: 2025-08-14.

### Integration

Integration with our existing api-gateway is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt Terraform.

**Rationale:** Agreed to sunset the legacy Python service by end of Q2.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Tomas Novak
- [ ] Prototype integration with data-warehouse
- [ ] Get sign-off from Frank Müller
