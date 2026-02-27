# Meeting Notes — Apex Security — 2024-08-18

**Date:** 2024-08-18
**Attendees:** Kofi Mensah, Bob Martins, David Park, Quinn Murphy, Sofia Andersen
**Project:** Apex Security

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Quinn Murphy raised concerns about disk I/O bottleneck during bulk import.
Bob Martins explained that this was related to the recent changes in webhook-handler.

We discussed migrating to FastAPI for better performance. Kofi Mensah had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and user-service was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- We will require code review from 2 engineers before merging.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Add structured logging with trace ids — **Kofi Mensah** — Due 2026-01-10
- [ ] Migrate the legacy monolith to microservices — **Sofia Andersen** — Due 2026-02-26
- [ ] Implement circuit breakers for downstream calls — **Kofi Mensah** — Due 2026-02-03
- [ ] Set up alerting for p99 latency — **Kofi Mensah** — Due 2026-01-16

## Notes

Stack: FastAPI, Kubernetes, SQLite
Services involved: webhook-handler, notification-service, user-service
