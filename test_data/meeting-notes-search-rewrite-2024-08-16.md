# Meeting Notes — Search Rewrite — 2024-08-16

**Date:** 2024-08-16
**Attendees:** Mohamed Al-Rashid, Laura Bianchi, Clara Johansson, Frank Müller
**Project:** Search Rewrite

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Frank Müller raised concerns about token expiry edge case when clock skew > 30s.
Mohamed Al-Rashid explained that this was related to the recent changes in auth-service.

We discussed migrating to Helm for better performance. Mohamed Al-Rashid had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and report-generator was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Frank Müller** — Due 2026-01-17
- [ ] Benchmark the new storage backend — **Laura Bianchi** — Due 2026-01-22
- [ ] Write runbooks for the on-call team — **Mohamed Al-Rashid** — Due 2026-01-09
- [ ] Document the deployment process — **Laura Bianchi** — Due 2026-02-05

## Notes

Stack: Helm
Services involved: auth-service, cache-layer, report-generator
