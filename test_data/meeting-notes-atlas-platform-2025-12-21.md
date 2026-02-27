# Meeting Notes — Atlas Platform — 2025-12-21

**Date:** 2025-12-21
**Attendees:** Ravi Sharma, Quinn Murphy, Henrik Larsen, Oscar Lindberg
**Project:** Atlas Platform

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Ravi Sharma raised concerns about memory leak in the worker pool.
Henrik Larsen explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Go for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and audit-logger was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- Will use Redis for session storage — simple and battle-tested.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Add rate limiting to the public api — **Oscar Lindberg** — Due 2026-01-17
- [ ] Migrate the legacy monolith to microservices — **Henrik Larsen** — Due 2026-02-09

## Notes

Stack: Go, RabbitMQ
Services involved: webhook-handler, event-bus, audit-logger
