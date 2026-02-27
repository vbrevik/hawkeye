# Meeting Notes — Search Rewrite — 2024-12-05

**Date:** 2024-12-05
**Attendees:** Tomas Novak, Nadia Kovač
**Project:** Search Rewrite

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Nadia Kovač raised concerns about retry storm after upstream timeout.
Nadia Kovač explained that this was related to the recent changes in scheduler.

We discussed migrating to Helm for better performance. Nadia Kovač had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and report-generator was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Add rate limiting to the public api — **Nadia Kovač** — Due 2026-02-01
- [ ] Set up alerting for p99 latency — **Nadia Kovač** — Due 2026-02-18

## Notes

Stack: Helm
Services involved: scheduler, analytics-pipeline, report-generator
