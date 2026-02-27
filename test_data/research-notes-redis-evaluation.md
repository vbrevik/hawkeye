# Research Notes — Redis Evaluation

**Author:** David Park
**Date:** 2024-04-21

## Purpose

Evaluating Redis for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for data-warehouse.

## Key Findings

### Performance

Initial benchmarks show Redis handles approximately 41,434 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is similar — roughly
477MB under load compared to our current 286MB.

### Operational Complexity

Redis requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Redis community is growing.
Documentation quality is good.
Last major release: 2025-04-18.

### Integration

Integration with our existing media-uploader is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Evaluate further Redis.

**Rationale:** PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Tomas Novak
- [ ] Prototype integration with media-uploader
- [ ] Get sign-off from Frank Müller
