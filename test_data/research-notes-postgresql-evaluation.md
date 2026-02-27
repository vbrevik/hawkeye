# Research Notes — PostgreSQL Evaluation

**Author:** Bob Martins
**Date:** 2024-05-25

## Purpose

Evaluating PostgreSQL for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for event-bus.

## Key Findings

### Performance

Initial benchmarks show PostgreSQL handles approximately 11,626 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is similar — roughly
151MB under load compared to our current 240MB.

### Operational Complexity

PostgreSQL requires moderate operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The PostgreSQL community is growing.
Documentation quality is adequate.
Last major release: 2025-07-23.

### Integration

Integration with our existing media-uploader is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Adopt PostgreSQL.

**Rationale:** We will use Rust for the new service due to memory safety and performance.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Henrik Larsen
- [ ] Prototype integration with analytics-pipeline
- [ ] Get sign-off from Priya Patel
