# Meeting Notes — Lighthouse CMS — 2023-06-21

**Date:** 2023-06-21
**Attendees:** Clara Johansson, Nadia Kovač, Laura Bianchi, Tomas Novak
**Project:** Lighthouse CMS

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Clara Johansson raised concerns about retry storm after upstream timeout.
Laura Bianchi explained that this was related to the recent changes in cache-layer.

We discussed migrating to Nginx for better performance. Tomas Novak had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and cache-layer was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Document the deployment process — **Tomas Novak** — Due 2026-01-14
- [ ] Benchmark the new storage backend — **Nadia Kovač** — Due 2026-01-07

## Notes

Stack: Nginx, Redis, React
Services involved: cache-layer
