# Meeting Notes — Apex Security — 2025-02-18

**Date:** 2025-02-18
**Attendees:** Clara Johansson, Frank Müller
**Project:** Apex Security

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Clara Johansson raised concerns about slow query on the user lookup table (missing index).
Clara Johansson explained that this was related to the recent changes in report-generator.

We discussed migrating to FastAPI for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and search-service was identified as a risk.
Frank Müller will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- We will require code review from 2 engineers before merging.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Frank Müller** — Due 2026-02-02
- [ ] Add rate limiting to the public api — **Clara Johansson** — Due 2026-01-30

## Notes

Stack: FastAPI, Prometheus, gRPC
Services involved: report-generator, search-service
