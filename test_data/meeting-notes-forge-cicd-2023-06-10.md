# Meeting Notes — Forge CI/CD — 2023-06-10

**Date:** 2023-06-10
**Attendees:** Jae-won Kim, Frank Müller, Gina Torres, Henrik Larsen, Nadia Kovač
**Project:** Forge CI/CD

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Forge CI/CD. Gina Torres raised concerns about goroutine leak in the WebSocket handler.
Henrik Larsen explained that this was related to the recent changes in event-bus.

We discussed migrating to SQLite for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and payment-processor was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Write runbooks for the on-call team — **Henrik Larsen** — Due 2026-01-28
- [ ] Document the deployment process — **Nadia Kovač** — Due 2026-02-09

## Notes

Stack: SQLite, Nginx, Redis
Services involved: event-bus, media-uploader, payment-processor
