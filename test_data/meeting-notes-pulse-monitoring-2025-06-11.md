# Meeting Notes — Pulse Monitoring — 2025-06-11

**Date:** 2025-06-11
**Attendees:** Clara Johansson, Priya Patel, Laura Bianchi, Nadia Kovač, Kofi Mensah
**Project:** Pulse Monitoring

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Clara Johansson raised concerns about retry storm after upstream timeout.
Laura Bianchi explained that this was related to the recent changes in data-warehouse.

We discussed migrating to GraphQL for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and scheduler was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Review and rotate all secrets in vault — **Nadia Kovač** — Due 2026-02-25
- [ ] Add structured logging with trace ids — **Kofi Mensah** — Due 2026-02-22
- [ ] Write runbooks for the on-call team — **Clara Johansson** — Due 2026-02-12

## Notes

Stack: GraphQL
Services involved: data-warehouse, webhook-handler, scheduler
