# Meeting Notes — Project Phoenix — 2023-10-03

**Date:** 2023-10-03
**Attendees:** Bob Martins, Clara Johansson
**Project:** Project Phoenix

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Bob Martins raised concerns about SSL certificate not renewing automatically.
Clara Johansson explained that this was related to the recent changes in search-service.

We discussed migrating to Axum for better performance. Clara Johansson had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and audit-logger was identified as a risk.
Bob Martins will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Document the deployment process — **Clara Johansson** — Due 2026-02-12
- [ ] Set up alerting for p99 latency — **Clara Johansson** — Due 2026-02-01
- [ ] Migrate the legacy monolith to microservices — **Clara Johansson** — Due 2026-02-04

## Notes

Stack: Axum
Services involved: search-service, audit-logger
