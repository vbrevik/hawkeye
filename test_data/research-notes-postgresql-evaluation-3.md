# Research Notes — PostgreSQL Evaluation

**Author:** Tomas Novak
**Date:** 2025-08-06

## Purpose

Evaluating PostgreSQL for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for webhook-handler.

## Key Findings

### Performance

Initial benchmarks show PostgreSQL handles approximately 68,998 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is higher — roughly
53MB under load compared to our current 415MB.

### Operational Complexity

PostgreSQL requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The PostgreSQL community is growing.
Documentation quality is good.
Last major release: 2025-07-01.

### Integration

Integration with our existing api-gateway is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt PostgreSQL.

**Rationale:** Chose gRPC over REST for the internal service mesh.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Frank Müller
- [ ] Prototype integration with data-warehouse
- [ ] Get sign-off from Laura Bianchi
