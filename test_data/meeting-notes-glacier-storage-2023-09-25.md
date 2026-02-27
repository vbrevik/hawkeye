# Meeting Notes — Glacier Storage — 2023-09-25

**Date:** 2023-09-25
**Attendees:** Isabelle Dupont, Kofi Mensah, Mohamed Al-Rashid, Sofia Andersen
**Project:** Glacier Storage

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Isabelle Dupont raised concerns about cache invalidation not propagating across regions.
Sofia Andersen explained that this was related to the recent changes in scheduler.

We discussed migrating to Rust for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Add rate limiting to the public api — **Sofia Andersen** — Due 2026-01-08
- [ ] Implement circuit breakers for downstream calls — **Mohamed Al-Rashid** — Due 2026-02-19

## Notes

Stack: Rust
Services involved: scheduler
