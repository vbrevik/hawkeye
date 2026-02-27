# Meeting Notes — Core Refactor Q1 — 2023-02-17

**Date:** 2023-02-17
**Attendees:** Ravi Sharma, Oscar Lindberg, Alice Chen, Nadia Kovač
**Project:** Core Refactor Q1

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Oscar Lindberg raised concerns about memory leak in the worker pool.
Alice Chen explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to FastAPI for better performance. Nadia Kovač had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and cache-layer was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Chose gRPC over REST for the internal service mesh.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Write runbooks for the on-call team — **Alice Chen** — Due 2026-01-01
- [ ] Add rate limiting to the public api — **Nadia Kovač** — Due 2026-01-13
- [ ] Add structured logging with trace ids — **Alice Chen** — Due 2026-01-06

## Notes

Stack: FastAPI
Services involved: analytics-pipeline, cache-layer
