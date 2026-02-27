# Meeting Notes — Nexus API — 2025-10-13

**Date:** 2025-10-13
**Attendees:** Clara Johansson, Henrik Larsen, Laura Bianchi, Kofi Mensah
**Project:** Nexus API

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Laura Bianchi raised concerns about race condition during concurrent writes.
Clara Johansson explained that this was related to the recent changes in payment-processor.

We discussed migrating to Grafana for better performance. Clara Johansson had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and payment-processor was identified as a risk.
Kofi Mensah will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Set up alerting for p99 latency — **Kofi Mensah** — Due 2026-02-16
- [ ] Add structured logging with trace ids — **Kofi Mensah** — Due 2026-01-11
- [ ] Refactor the authentication middleware — **Kofi Mensah** — Due 2026-02-01
- [ ] Migrate the legacy monolith to microservices — **Kofi Mensah** — Due 2026-01-23

## Notes

Stack: Grafana
Services involved: payment-processor
