# Meeting Notes — Lighthouse CMS — 2025-06-01

**Date:** 2025-06-01
**Attendees:** Nadia Kovač, Oscar Lindberg, Bob Martins, David Park, Ravi Sharma
**Project:** Lighthouse CMS

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Oscar Lindberg raised concerns about race condition during concurrent writes.
David Park explained that this was related to the recent changes in event-bus.

We discussed migrating to Terraform for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and data-warehouse was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Nadia Kovač** — Due 2026-02-11
- [ ] Document the deployment process — **Ravi Sharma** — Due 2026-02-23

## Notes

Stack: Terraform, GraphQL
Services involved: event-bus, notification-service, data-warehouse
