# Meeting Notes — Search Rewrite — 2024-03-29

**Date:** 2024-03-29
**Attendees:** Kofi Mensah, Tomas Novak, Laura Bianchi
**Project:** Search Rewrite

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Tomas Novak raised concerns about goroutine leak in the WebSocket handler.
Kofi Mensah explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to PostgreSQL for better performance. Tomas Novak had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and payment-processor was identified as a risk.
Tomas Novak will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Benchmark the new storage backend — **Laura Bianchi** — Due 2026-02-16
- [ ] Refactor the authentication middleware — **Tomas Novak** — Due 2026-01-02
- [ ] Add rate limiting to the public api — **Laura Bianchi** — Due 2026-02-22

## Notes

Stack: PostgreSQL, RabbitMQ
Services involved: analytics-pipeline, event-bus, payment-processor
