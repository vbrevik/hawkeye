# Meeting Notes — Stratos Deploy — 2024-10-21

**Date:** 2024-10-21
**Attendees:** Oscar Lindberg, Gina Torres, Quinn Murphy, Priya Patel, Frank Müller
**Project:** Stratos Deploy

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Oscar Lindberg raised concerns about slow query on the user lookup table (missing index).
Priya Patel explained that this was related to the recent changes in scheduler.

We discussed migrating to Rust for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Will use Redis for session storage — simple and battle-tested.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Priya Patel** — Due 2026-01-27
- [ ] Write runbooks for the on-call team — **Frank Müller** — Due 2026-01-13

## Notes

Stack: Rust, Kubernetes
Services involved: scheduler
