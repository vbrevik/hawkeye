# Research Notes — Terraform Evaluation

**Author:** Clara Johansson
**Date:** 2024-10-16

## Purpose

Evaluating Terraform for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for api-gateway.

## Key Findings

### Performance

Initial benchmarks show Terraform handles approximately 55,868 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is higher — roughly
332MB under load compared to our current 266MB.

### Operational Complexity

Terraform requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Terraform community is mature and stable.
Documentation quality is good.
Last major release: 2025-01-07.

### Integration

Integration with our existing scheduler is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt Terraform.

**Rationale:** PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Nadia Kovač
- [ ] Prototype integration with notification-service
- [ ] Get sign-off from Elena Rossi
