# Meeting Notes — Project Phoenix — 2023-05-19

**Date:** 2023-05-19
**Attendees:** Gina Torres, Bob Martins, Sofia Andersen, Mohamed Al-Rashid
**Project:** Project Phoenix

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Bob Martins raised concerns about cache invalidation not propagating across regions.
Sofia Andersen explained that this was related to the recent changes in api-gateway.

We discussed migrating to Vault for better performance. Mohamed Al-Rashid had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and user-service was identified as a risk.
Gina Torres will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- We will require code review from 2 engineers before merging.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Review and rotate all secrets in vault — **Sofia Andersen** — Due 2026-02-14
- [ ] Add rate limiting to the public api — **Mohamed Al-Rashid** — Due 2026-02-16
- [ ] Migrate the legacy monolith to microservices — **Bob Martins** — Due 2026-01-04

## Notes

Stack: Vault
Services involved: api-gateway, user-service
