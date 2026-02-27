# Meeting Notes — Auth Overhaul — 2025-06-06

**Date:** 2025-06-06
**Attendees:** Priya Patel, Alice Chen, Mohamed Al-Rashid, Nadia Kovač, Frank Müller
**Project:** Auth Overhaul

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Nadia Kovač raised concerns about goroutine leak in the WebSocket handler.
Frank Müller explained that this was related to the recent changes in search-service.

We discussed migrating to DynamoDB for better performance. Nadia Kovač had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and search-service was identified as a risk.
Priya Patel will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Write runbooks for the on-call team — **Priya Patel** — Due 2026-01-18
- [ ] Add structured logging with trace ids — **Nadia Kovač** — Due 2026-01-09
- [ ] Migrate the legacy monolith to microservices — **Nadia Kovač** — Due 2026-01-13

## Notes

Stack: DynamoDB, Docker
Services involved: search-service
