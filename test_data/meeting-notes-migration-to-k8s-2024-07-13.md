# Meeting Notes — Migration to K8s — 2024-07-13

**Date:** 2024-07-13
**Attendees:** Priya Patel, Henrik Larsen, Laura Bianchi, Mohamed Al-Rashid, Nadia Kovač
**Project:** Migration to K8s

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Migration to K8s. Laura Bianchi raised concerns about cache invalidation not propagating across regions.
Mohamed Al-Rashid explained that this was related to the recent changes in report-generator.

We discussed migrating to Redis for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Review and rotate all secrets in vault — **Laura Bianchi** — Due 2026-01-24
- [ ] Add rate limiting to the public api — **Priya Patel** — Due 2026-01-06
- [ ] Write runbooks for the on-call team — **Priya Patel** — Due 2026-01-20
- [ ] Document the deployment process — **Mohamed Al-Rashid** — Due 2026-01-24

## Notes

Stack: Redis
Services involved: report-generator
