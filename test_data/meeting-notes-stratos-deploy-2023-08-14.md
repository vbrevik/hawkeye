# Meeting Notes — Stratos Deploy — 2023-08-14

**Date:** 2023-08-14
**Attendees:** Frank Müller, Gina Torres, Bob Martins
**Project:** Stratos Deploy

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Frank Müller raised concerns about token expiry edge case when clock skew > 30s.
Frank Müller explained that this was related to the recent changes in user-service.

We discussed migrating to Elasticsearch for better performance. Gina Torres had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and user-service was identified as a risk.
Frank Müller will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Agreed to sunset the legacy Python service by end of Q2.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Add structured logging with trace ids — **Bob Martins** — Due 2026-01-16
- [ ] Add rate limiting to the public api — **Gina Torres** — Due 2026-02-01
- [ ] Migrate the legacy monolith to microservices — **Bob Martins** — Due 2026-02-18

## Notes

Stack: Elasticsearch
Services involved: user-service
