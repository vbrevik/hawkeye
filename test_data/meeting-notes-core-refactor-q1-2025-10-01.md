# Meeting Notes — Core Refactor Q1 — 2025-10-01

**Date:** 2025-10-01
**Attendees:** Quinn Murphy, Laura Bianchi, Ravi Sharma, Tomas Novak
**Project:** Core Refactor Q1

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Laura Bianchi raised concerns about disk I/O bottleneck during bulk import.
Ravi Sharma explained that this was related to the recent changes in user-service.

We discussed migrating to Rust for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and user-service was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Refactor the authentication middleware — **Tomas Novak** — Due 2026-01-05
- [ ] Implement circuit breakers for downstream calls — **Ravi Sharma** — Due 2026-01-17
- [ ] Document the deployment process — **Tomas Novak** — Due 2026-02-11
- [ ] Add rate limiting to the public api — **Laura Bianchi** — Due 2026-01-26

## Notes

Stack: Rust, S3
Services involved: user-service
