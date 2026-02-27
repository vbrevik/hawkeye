# Meeting Notes — Atlas Platform — 2025-08-06

**Date:** 2025-08-06
**Attendees:** Henrik Larsen, Gina Torres
**Project:** Atlas Platform

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Gina Torres raised concerns about slow query on the user lookup table (missing index).
Henrik Larsen explained that this was related to the recent changes in search-service.

We discussed migrating to PostgreSQL for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and webhook-handler was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Adopted conventional commits across all repositories.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Gina Torres** — Due 2026-01-07
- [ ] Set up alerting for p99 latency — **Henrik Larsen** — Due 2026-01-10
- [ ] Write runbooks for the on-call team — **Gina Torres** — Due 2026-02-05

## Notes

Stack: PostgreSQL, Celery, Axum, RabbitMQ
Services involved: search-service, webhook-handler
