# Meeting Notes — Project Phoenix — 2025-09-17

**Date:** 2025-09-17
**Attendees:** Kofi Mensah, Clara Johansson
**Project:** Project Phoenix

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Kofi Mensah raised concerns about retry storm after upstream timeout.
Clara Johansson explained that this was related to the recent changes in scheduler.

We discussed migrating to Rust for better performance. Kofi Mensah had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and data-warehouse was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Review and rotate all secrets in vault — **Kofi Mensah** — Due 2026-02-07
- [ ] Benchmark the new storage backend — **Clara Johansson** — Due 2026-01-31
- [ ] Implement circuit breakers for downstream calls — **Kofi Mensah** — Due 2026-01-19

## Notes

Stack: Rust, Celery, RabbitMQ
Services involved: scheduler, data-warehouse
