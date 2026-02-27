# Meeting Notes — Auth Overhaul — 2026-02-24

**Date:** 2026-02-24
**Attendees:** Ravi Sharma, Mohamed Al-Rashid, Henrik Larsen
**Project:** Auth Overhaul

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Mohamed Al-Rashid raised concerns about slow query on the user lookup table (missing index).
Henrik Larsen explained that this was related to the recent changes in report-generator.

We discussed migrating to TypeScript for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- We will use Rust for the new service due to memory safety and performance.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Review and rotate all secrets in vault — **Henrik Larsen** — Due 2026-01-24
- [ ] Set up alerting for p99 latency — **Henrik Larsen** — Due 2026-02-04
- [ ] Refactor the authentication middleware — **Henrik Larsen** — Due 2026-02-08

## Notes

Stack: TypeScript, S3, Axum
Services involved: report-generator
