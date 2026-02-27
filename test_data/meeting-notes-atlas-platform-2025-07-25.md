# Meeting Notes — Atlas Platform — 2025-07-25

**Date:** 2025-07-25
**Attendees:** Priya Patel, Ravi Sharma
**Project:** Atlas Platform

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Priya Patel raised concerns about race condition during concurrent writes.
Ravi Sharma explained that this was related to the recent changes in search-service.

We discussed migrating to React for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and search-service was identified as a risk.
Priya Patel will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Ravi Sharma** — Due 2026-02-25
- [ ] Migrate the legacy monolith to microservices — **Ravi Sharma** — Due 2026-01-03
- [ ] Write runbooks for the on-call team — **Ravi Sharma** — Due 2026-01-24
- [ ] Document the deployment process — **Ravi Sharma** — Due 2026-02-17

## Notes

Stack: React, Helm, gRPC
Services involved: search-service
