# Meeting Notes — Lighthouse CMS — 2024-07-12

**Date:** 2024-07-12
**Attendees:** Laura Bianchi, Bob Martins, Jae-won Kim, Ravi Sharma, Clara Johansson
**Project:** Lighthouse CMS

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Jae-won Kim raised concerns about race condition during concurrent writes.
Clara Johansson explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Terraform for better performance. Clara Johansson had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and audit-logger was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- Decided to go with a pull-based deployment model using ArgoCD.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Bob Martins** — Due 2026-01-01
- [ ] Benchmark the new storage backend — **Bob Martins** — Due 2026-02-03
- [ ] Set up alerting for p99 latency — **Jae-won Kim** — Due 2026-01-24
- [ ] Review and rotate all secrets in vault — **Ravi Sharma** — Due 2026-01-27

## Notes

Stack: Terraform, RabbitMQ, Celery
Services involved: webhook-handler, audit-logger
