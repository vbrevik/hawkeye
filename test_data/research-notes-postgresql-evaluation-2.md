# Research Notes — PostgreSQL Evaluation

**Author:** David Park
**Date:** 2025-11-04

## Purpose

Evaluating PostgreSQL for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for notification-service.

## Key Findings

### Performance

Initial benchmarks show PostgreSQL handles approximately 87,565 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is lower — roughly
235MB under load compared to our current 180MB.

### Operational Complexity

PostgreSQL requires significant operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The PostgreSQL community is growing.
Documentation quality is adequate.
Last major release: 2025-08-15.

### Integration

Integration with our existing scheduler is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt PostgreSQL.

**Rationale:** PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Elena Rossi
- [ ] Prototype integration with media-uploader
- [ ] Get sign-off from Laura Bianchi
