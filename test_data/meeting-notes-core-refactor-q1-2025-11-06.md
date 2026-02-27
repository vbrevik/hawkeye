# Meeting Notes — Core Refactor Q1 — 2025-11-06

**Date:** 2025-11-06
**Attendees:** Clara Johansson, Isabelle Dupont, Nadia Kovač
**Project:** Core Refactor Q1

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Isabelle Dupont raised concerns about retry storm after upstream timeout.
Isabelle Dupont explained that this was related to the recent changes in search-service.

We discussed migrating to GraphQL for better performance. Nadia Kovač had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and event-bus was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Write runbooks for the on-call team — **Isabelle Dupont** — Due 2026-01-30
- [ ] Document the deployment process — **Isabelle Dupont** — Due 2026-02-25

## Notes

Stack: GraphQL, ArgoCD
Services involved: search-service, media-uploader, event-bus
