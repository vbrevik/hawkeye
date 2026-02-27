# Meeting Notes — Pulse Monitoring — 2023-06-09

**Date:** 2023-06-09
**Attendees:** Tomas Novak, Bob Martins, Kofi Mensah, Alice Chen
**Project:** Pulse Monitoring

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Kofi Mensah raised concerns about disk I/O bottleneck during bulk import.
Kofi Mensah explained that this was related to the recent changes in scheduler.

We discussed migrating to Go for better performance. Tomas Novak had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and user-service was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Document the deployment process — **Bob Martins** — Due 2026-02-06
- [ ] Set up alerting for p99 latency — **Tomas Novak** — Due 2026-02-12
- [ ] Review and rotate all secrets in vault — **Kofi Mensah** — Due 2026-02-21

## Notes

Stack: Go, PostgreSQL, Celery
Services involved: scheduler, user-service
