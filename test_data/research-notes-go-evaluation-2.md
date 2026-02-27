# Research Notes — Go Evaluation

**Author:** Laura Bianchi
**Date:** 2023-04-22

## Purpose

Evaluating Go for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for media-uploader.

## Key Findings

### Performance

Initial benchmarks show Go handles approximately 60,603 req/s
under our expected load profile. This is significantly better than our current setup.

Memory usage is higher — roughly
105MB under load compared to our current 153MB.

### Operational Complexity

Go requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Go community is growing.
Documentation quality is adequate.
Last major release: 2026-02-03.

### Integration

Integration with our existing webhook-handler is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Evaluate further Go.

**Rationale:** We will use Rust for the new service due to memory safety and performance.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Henrik Larsen
- [ ] Prototype integration with user-service
- [ ] Get sign-off from Frank Müller
