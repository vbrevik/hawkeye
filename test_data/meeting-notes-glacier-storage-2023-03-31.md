# Meeting Notes — Glacier Storage — 2023-03-31

**Date:** 2023-03-31
**Attendees:** Isabelle Dupont, Ravi Sharma, Jae-won Kim
**Project:** Glacier Storage

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Ravi Sharma raised concerns about goroutine leak in the WebSocket handler.
Ravi Sharma explained that this was related to the recent changes in scheduler.

We discussed migrating to Vault for better performance. Isabelle Dupont had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Agreed to sunset the legacy Python service by end of Q2.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Benchmark the new storage backend — **Isabelle Dupont** — Due 2026-02-02
- [ ] Add rate limiting to the public api — **Isabelle Dupont** — Due 2026-01-30
- [ ] Set up alerting for p99 latency — **Isabelle Dupont** — Due 2026-01-05

## Notes

Stack: Vault
Services involved: scheduler
