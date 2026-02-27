# Research Notes — Rust Evaluation

**Author:** Tomas Novak
**Date:** 2023-02-22

## Purpose

Evaluating Rust for potential adoption in our stack. Specifically looking at
whether it can replace our current solution for api-gateway.

## Key Findings

### Performance

Initial benchmarks show Rust handles approximately 74,550 req/s
under our expected load profile. This is comparable to our current setup.

Memory usage is similar — roughly
434MB under load compared to our current 517MB.

### Operational Complexity

Rust requires minimal operational overhead.
The main concerns are:

- Configuration management
- Monitoring and observability integration
- Upgrade path and version compatibility

### Community and Support

The Rust community is growing.
Documentation quality is good.
Last major release: 2025-11-13.

### Integration

Integration with our existing media-uploader is straightforward. We can
use the official client library with minimal changes to our codebase.

## Recommendation

Do not adopt Rust.

**Rationale:** We will require code review from 2 engineers before merging.

## Next Steps

- [ ] Run extended load test over 48h
- [ ] Review security posture with Nadia Kovač
- [ ] Prototype integration with media-uploader
- [ ] Get sign-off from Gina Torres
