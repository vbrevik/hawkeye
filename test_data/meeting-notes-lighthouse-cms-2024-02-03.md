# Meeting Notes — Lighthouse CMS — 2024-02-03

**Date:** 2024-02-03
**Attendees:** Alice Chen, Oscar Lindberg, Clara Johansson, Henrik Larsen, Jae-won Kim
**Project:** Lighthouse CMS

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Oscar Lindberg raised concerns about retry storm after upstream timeout.
Clara Johansson explained that this was related to the recent changes in cache-layer.

We discussed migrating to S3 for better performance. Alice Chen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and audit-logger was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Adopted conventional commits across all repositories.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Add rate limiting to the public api — **Alice Chen** — Due 2026-02-13
- [ ] Document the deployment process — **Oscar Lindberg** — Due 2026-01-01

## Notes

Stack: S3
Services involved: cache-layer, audit-logger
