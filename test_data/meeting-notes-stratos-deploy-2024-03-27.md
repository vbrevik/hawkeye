# Meeting Notes — Stratos Deploy — 2024-03-27

**Date:** 2024-03-27
**Attendees:** Clara Johansson, Kofi Mensah, Bob Martins, Gina Torres
**Project:** Stratos Deploy

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Gina Torres raised concerns about slow query on the user lookup table (missing index).
Gina Torres explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Vault for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and media-uploader was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.
- Agreed to sunset the legacy Python service by end of Q2.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Refactor the authentication middleware — **Bob Martins** — Due 2026-01-26
- [ ] Add rate limiting to the public api — **Kofi Mensah** — Due 2026-02-15

## Notes

Stack: Vault
Services involved: webhook-handler, event-bus, media-uploader
