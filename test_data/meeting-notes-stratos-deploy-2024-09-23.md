# Meeting Notes — Stratos Deploy — 2024-09-23

**Date:** 2024-09-23
**Attendees:** David Park, Clara Johansson, Frank Müller, Jae-won Kim
**Project:** Stratos Deploy

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Frank Müller raised concerns about token expiry edge case when clock skew > 30s.
Clara Johansson explained that this was related to the recent changes in data-warehouse.

We discussed migrating to RabbitMQ for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and cache-layer was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- We will use Rust for the new service due to memory safety and performance.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Jae-won Kim** — Due 2026-02-05
- [ ] Add rate limiting to the public api — **Clara Johansson** — Due 2026-01-07
- [ ] Implement circuit breakers for downstream calls — **Jae-won Kim** — Due 2026-01-07

## Notes

Stack: RabbitMQ
Services involved: data-warehouse, cache-layer
