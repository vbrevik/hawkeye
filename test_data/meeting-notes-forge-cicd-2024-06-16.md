# Meeting Notes — Forge CI/CD — 2024-06-16

**Date:** 2024-06-16
**Attendees:** Gina Torres, Oscar Lindberg, Kofi Mensah, Ravi Sharma
**Project:** Forge CI/CD

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Forge CI/CD. Gina Torres raised concerns about memory leak in the worker pool.
Oscar Lindberg explained that this was related to the recent changes in cache-layer.

We discussed migrating to DynamoDB for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and analytics-pipeline was identified as a risk.
Gina Torres will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Add rate limiting to the public api — **Oscar Lindberg** — Due 2026-01-16
- [ ] Add structured logging with trace ids — **Oscar Lindberg** — Due 2026-01-31
- [ ] Refactor the authentication middleware — **Gina Torres** — Due 2026-02-06

## Notes

Stack: DynamoDB, Redis
Services involved: cache-layer, auth-service, analytics-pipeline
