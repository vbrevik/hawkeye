# Meeting Notes — Migration to K8s — 2023-08-09

**Date:** 2023-08-09
**Attendees:** Elena Rossi, Tomas Novak
**Project:** Migration to K8s

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Tomas Novak raised concerns about slow query on the user lookup table (missing index).
Elena Rossi explained that this was related to the recent changes in report-generator.

We discussed migrating to Docker for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and data-warehouse was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Tomas Novak** — Due 2026-02-19
- [ ] Refactor the authentication middleware — **Elena Rossi** — Due 2026-02-25
- [ ] Set up alerting for p99 latency — **Tomas Novak** — Due 2026-02-14
- [ ] Document the deployment process — **Elena Rossi** — Due 2026-02-12

## Notes

Stack: Docker, Nginx, Redis, Axum
Services involved: report-generator, auth-service, data-warehouse
