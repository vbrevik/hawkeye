# Meeting Notes — Core Refactor Q1 — 2024-10-30

**Date:** 2024-10-30
**Attendees:** Kofi Mensah, Priya Patel, Isabelle Dupont
**Project:** Core Refactor Q1

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Priya Patel raised concerns about memory leak in the worker pool.
Priya Patel explained that this was related to the recent changes in search-service.

We discussed migrating to Nginx for better performance. Isabelle Dupont had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and payment-processor was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- Adopted conventional commits across all repositories.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Set up alerting for p99 latency — **Priya Patel** — Due 2026-02-18
- [ ] Implement circuit breakers for downstream calls — **Kofi Mensah** — Due 2026-02-09

## Notes

Stack: Nginx, Helm
Services involved: search-service, audit-logger, payment-processor
