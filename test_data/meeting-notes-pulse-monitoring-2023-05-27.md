# Meeting Notes — Pulse Monitoring — 2023-05-27

**Date:** 2023-05-27
**Attendees:** Elena Rossi, Priya Patel
**Project:** Pulse Monitoring

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Priya Patel raised concerns about SSL certificate not renewing automatically.
Priya Patel explained that this was related to the recent changes in search-service.

We discussed migrating to GraphQL for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and audit-logger was identified as a risk.
Priya Patel will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Decided to go with a pull-based deployment model using ArgoCD.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Review and rotate all secrets in vault — **Elena Rossi** — Due 2026-01-12
- [ ] Add structured logging with trace ids — **Priya Patel** — Due 2026-02-08

## Notes

Stack: GraphQL, Vault
Services involved: search-service, data-warehouse, audit-logger
