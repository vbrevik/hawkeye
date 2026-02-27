# Meeting Notes — Forge CI/CD — 2025-02-28

**Date:** 2025-02-28
**Attendees:** Elena Rossi, Kofi Mensah
**Project:** Forge CI/CD

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Forge CI/CD. Elena Rossi raised concerns about goroutine leak in the WebSocket handler.
Kofi Mensah explained that this was related to the recent changes in search-service.

We discussed migrating to TypeScript for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and search-service was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Refactor the authentication middleware — **Kofi Mensah** — Due 2026-01-13
- [ ] Review and rotate all secrets in vault — **Elena Rossi** — Due 2026-02-01
- [ ] Migrate the legacy monolith to microservices — **Kofi Mensah** — Due 2026-01-11

## Notes

Stack: TypeScript, Kafka
Services involved: search-service
