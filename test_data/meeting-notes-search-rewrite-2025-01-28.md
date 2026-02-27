# Meeting Notes — Search Rewrite — 2025-01-28

**Date:** 2025-01-28
**Attendees:** Oscar Lindberg, Alice Chen, Henrik Larsen
**Project:** Search Rewrite

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Henrik Larsen raised concerns about disk I/O bottleneck during bulk import.
Oscar Lindberg explained that this was related to the recent changes in payment-processor.

We discussed migrating to PostgreSQL for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and event-bus was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Will use Redis for session storage — simple and battle-tested.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Add rate limiting to the public api — **Henrik Larsen** — Due 2026-01-26
- [ ] Write runbooks for the on-call team — **Oscar Lindberg** — Due 2026-02-25

## Notes

Stack: PostgreSQL, Terraform, Nginx
Services involved: payment-processor, audit-logger, event-bus
