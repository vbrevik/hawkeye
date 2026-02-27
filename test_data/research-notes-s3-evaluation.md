# Research Notes — S3 Evaluation

**Author:** Tomas Novak
**Date:** 2023-04-10

## Purpose

Evaluating S3 for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for auth-service.

## Key Findings

### Performance

Initial benchmarks show S3 handles approximately 91,765 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is similar — roughly
478MB under load compared to our current 110MB.

### Operational Complexity

S3 requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The S3 community is mature and stable.
Documentation quality is excellent.
Last major release: 2025-08-02.

### Integration

Integration with our existing event-bus is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt S3.

**Rationale:** PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Ravi Sharma
- [ ] Prototype integration with user-service
- [ ] Get sign-off from Priya Patel
