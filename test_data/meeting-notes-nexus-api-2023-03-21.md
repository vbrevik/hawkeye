# Meeting Notes — Nexus API — 2023-03-21

**Date:** 2023-03-21
**Attendees:** Priya Patel, Quinn Murphy, Oscar Lindberg, Elena Rossi
**Project:** Nexus API

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Oscar Lindberg raised concerns about flaky tests in the integration suite.
Priya Patel explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Vault for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and search-service was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.
- Decided to go with a pull-based deployment model using ArgoCD.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Set up alerting for p99 latency — **Priya Patel** — Due 2026-01-27
- [ ] Migrate the legacy monolith to microservices — **Oscar Lindberg** — Due 2026-02-20
- [ ] Review and rotate all secrets in vault — **Priya Patel** — Due 2026-02-08
- [ ] Write runbooks for the on-call team — **Elena Rossi** — Due 2026-01-17

## Notes

Stack: Vault, ArgoCD, Nginx
Services involved: webhook-handler, user-service, search-service
