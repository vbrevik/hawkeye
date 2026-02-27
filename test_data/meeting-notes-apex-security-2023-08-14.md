# Meeting Notes — Apex Security — 2023-08-14

**Date:** 2023-08-14
**Attendees:** Elena Rossi, Ravi Sharma, Alice Chen, Sofia Andersen
**Project:** Apex Security

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Sofia Andersen raised concerns about slow query on the user lookup table (missing index).
Alice Chen explained that this was related to the recent changes in payment-processor.

We discussed migrating to Go for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and cache-layer was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Benchmark the new storage backend — **Alice Chen** — Due 2026-01-27
- [ ] Add rate limiting to the public api — **Ravi Sharma** — Due 2026-02-05
- [ ] Refactor the authentication middleware — **Ravi Sharma** — Due 2026-01-01

## Notes

Stack: Go, ArgoCD
Services involved: payment-processor, analytics-pipeline, cache-layer
