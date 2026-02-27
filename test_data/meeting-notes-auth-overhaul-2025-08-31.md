# Meeting Notes — Auth Overhaul — 2025-08-31

**Date:** 2025-08-31
**Attendees:** Kofi Mensah, Clara Johansson
**Project:** Auth Overhaul

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Clara Johansson raised concerns about goroutine leak in the WebSocket handler.
Clara Johansson explained that this was related to the recent changes in cache-layer.

We discussed migrating to React for better performance. Clara Johansson had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and scheduler was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Add structured logging with trace ids — **Clara Johansson** — Due 2026-02-23
- [ ] Review and rotate all secrets in vault — **Clara Johansson** — Due 2026-02-03
- [ ] Document the deployment process — **Kofi Mensah** — Due 2026-02-26
- [ ] Migrate the legacy monolith to microservices — **Kofi Mensah** — Due 2026-01-11

## Notes

Stack: React
Services involved: cache-layer, scheduler
