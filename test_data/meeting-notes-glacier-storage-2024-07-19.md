# Meeting Notes — Glacier Storage — 2024-07-19

**Date:** 2024-07-19
**Attendees:** Oscar Lindberg, Clara Johansson
**Project:** Glacier Storage

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Oscar Lindberg raised concerns about retry storm after upstream timeout.
Oscar Lindberg explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Helm for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and webhook-handler was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- We will use Rust for the new service due to memory safety and performance.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Oscar Lindberg** — Due 2026-01-09
- [ ] Migrate the legacy monolith to microservices — **Clara Johansson** — Due 2026-02-21
- [ ] Document the deployment process — **Oscar Lindberg** — Due 2026-01-08

## Notes

Stack: Helm, TypeScript
Services involved: webhook-handler
