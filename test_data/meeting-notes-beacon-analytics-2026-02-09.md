# Meeting Notes — Beacon Analytics — 2026-02-09

**Date:** 2026-02-09
**Attendees:** Elena Rossi, Isabelle Dupont
**Project:** Beacon Analytics

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. Isabelle Dupont raised concerns about SSL certificate not renewing automatically.
Isabelle Dupont explained that this was related to the recent changes in payment-processor.

We discussed migrating to RabbitMQ for better performance. Isabelle Dupont had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and analytics-pipeline was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Benchmark the new storage backend — **Elena Rossi** — Due 2026-02-10
- [ ] Migrate the legacy monolith to microservices — **Isabelle Dupont** — Due 2026-02-23
- [ ] Write runbooks for the on-call team — **Isabelle Dupont** — Due 2026-01-03
- [ ] Document the deployment process — **Elena Rossi** — Due 2026-01-22

## Notes

Stack: RabbitMQ, DynamoDB
Services involved: payment-processor, report-generator, analytics-pipeline
