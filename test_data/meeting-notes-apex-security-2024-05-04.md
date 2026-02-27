# Meeting Notes — Apex Security — 2024-05-04

**Date:** 2024-05-04
**Attendees:** Henrik Larsen, Nadia Kovač, Bob Martins, Kofi Mensah, Priya Patel
**Project:** Apex Security

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Nadia Kovač raised concerns about goroutine leak in the WebSocket handler.
Nadia Kovač explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Kafka for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and webhook-handler was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Adopted conventional commits across all repositories.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Refactor the authentication middleware — **Bob Martins** — Due 2026-01-22
- [ ] Migrate the legacy monolith to microservices — **Priya Patel** — Due 2026-02-14
- [ ] Set up alerting for p99 latency — **Kofi Mensah** — Due 2026-02-11

## Notes

Stack: Kafka, Vault, GraphQL, Kubernetes
Services involved: webhook-handler
