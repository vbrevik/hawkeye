# Meeting Notes — Project Phoenix — 2025-10-09

**Date:** 2025-10-09
**Attendees:** Alice Chen, Gina Torres, Oscar Lindberg
**Project:** Project Phoenix

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Gina Torres raised concerns about cache invalidation not propagating across regions.
Oscar Lindberg explained that this was related to the recent changes in scheduler.

We discussed migrating to Axum for better performance. Alice Chen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and webhook-handler was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Set up alerting for p99 latency — **Alice Chen** — Due 2026-01-24
- [ ] Implement circuit breakers for downstream calls — **Gina Torres** — Due 2026-01-29

## Notes

Stack: Axum, Docker, Celery
Services involved: scheduler, cache-layer, webhook-handler
