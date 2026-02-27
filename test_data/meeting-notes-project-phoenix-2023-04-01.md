# Meeting Notes — Project Phoenix — 2023-04-01

**Date:** 2023-04-01
**Attendees:** Quinn Murphy, Bob Martins, Nadia Kovač, Tomas Novak, Mohamed Al-Rashid
**Project:** Project Phoenix

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Bob Martins raised concerns about race condition during concurrent writes.
Quinn Murphy explained that this was related to the recent changes in report-generator.

We discussed migrating to Grafana for better performance. Tomas Novak had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and payment-processor was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Bob Martins** — Due 2026-02-15
- [ ] Add rate limiting to the public api — **Tomas Novak** — Due 2026-02-24
- [ ] Refactor the authentication middleware — **Mohamed Al-Rashid** — Due 2026-02-17

## Notes

Stack: Grafana, Docker
Services involved: report-generator, data-warehouse, payment-processor
