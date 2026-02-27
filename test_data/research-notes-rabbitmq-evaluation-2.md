# Research Notes — RabbitMQ Evaluation

**Author:** Elena Rossi
**Date:** 2023-09-16

## Purpose

Evaluating RabbitMQ for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for data-warehouse.

## Key Findings

### Performance

Initial benchmarks show RabbitMQ handles approximately 79,391 req/s
under our expected load profile. This is slightly worse than our current setup.

Memory usage is higher — roughly
77MB under load compared to our current 570MB.

### Operational Complexity

RabbitMQ requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The RabbitMQ community is mature and stable.
Documentation quality is good.
Last major release: 2025-12-29.

### Integration

Integration with our existing scheduler is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt with caveats RabbitMQ.

**Rationale:** Adopted conventional commits across all repositories.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Nadia Kovač
- [ ] Prototype integration with report-generator
- [ ] Get sign-off from Oscar Lindberg
