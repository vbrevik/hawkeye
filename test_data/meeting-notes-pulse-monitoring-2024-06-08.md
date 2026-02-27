# Meeting Notes — Pulse Monitoring — 2024-06-08

**Date:** 2024-06-08
**Attendees:** Oscar Lindberg, Jae-won Kim, Isabelle Dupont
**Project:** Pulse Monitoring

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Oscar Lindberg raised concerns about memory leak in the worker pool.
Oscar Lindberg explained that this was related to the recent changes in scheduler.

We discussed migrating to gRPC for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and media-uploader was identified as a risk.
Isabelle Dupont will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Refactor the authentication middleware — **Isabelle Dupont** — Due 2026-01-11
- [ ] Write runbooks for the on-call team — **Jae-won Kim** — Due 2026-01-23

## Notes

Stack: gRPC, FastAPI, Kafka
Services involved: scheduler, media-uploader
