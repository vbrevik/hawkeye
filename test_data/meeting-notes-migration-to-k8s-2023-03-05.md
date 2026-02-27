# Meeting Notes — Migration to K8s — 2023-03-05

**Date:** 2023-03-05
**Attendees:** Jae-won Kim, Bob Martins, Isabelle Dupont
**Project:** Migration to K8s

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Isabelle Dupont raised concerns about retry storm after upstream timeout.
Isabelle Dupont explained that this was related to the recent changes in cache-layer.

We discussed migrating to Terraform for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and search-service was identified as a risk.
Isabelle Dupont will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Bob Martins** — Due 2026-02-04
- [ ] Add structured logging with trace ids — **Jae-won Kim** — Due 2026-01-28
- [ ] Benchmark the new storage backend — **Jae-won Kim** — Due 2026-02-17

## Notes

Stack: Terraform
Services involved: cache-layer, api-gateway, search-service
