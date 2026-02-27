# Meeting Notes — Migration to K8s — 2025-02-11

**Date:** 2025-02-11
**Attendees:** Henrik Larsen, Laura Bianchi, Tomas Novak
**Project:** Migration to K8s

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Henrik Larsen raised concerns about race condition during concurrent writes.
Tomas Novak explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to RabbitMQ for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and analytics-pipeline was identified as a risk.
Tomas Novak will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Review and rotate all secrets in vault — **Henrik Larsen** — Due 2026-01-03
- [ ] Refactor the authentication middleware — **Laura Bianchi** — Due 2026-02-02

## Notes

Stack: RabbitMQ, Redis
Services involved: analytics-pipeline
