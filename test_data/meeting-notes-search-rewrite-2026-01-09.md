# Meeting Notes — Search Rewrite — 2026-01-09

**Date:** 2026-01-09
**Attendees:** Quinn Murphy, Tomas Novak, Isabelle Dupont, Frank Müller
**Project:** Search Rewrite

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Frank Müller raised concerns about SSL certificate not renewing automatically.
Isabelle Dupont explained that this was related to the recent changes in scheduler.

We discussed migrating to RabbitMQ for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Frank Müller will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Set up alerting for p99 latency — **Quinn Murphy** — Due 2026-01-17
- [ ] Add rate limiting to the public api — **Quinn Murphy** — Due 2026-01-30
- [ ] Add structured logging with trace ids — **Isabelle Dupont** — Due 2026-02-13
- [ ] Refactor the authentication middleware — **Isabelle Dupont** — Due 2026-02-07

## Notes

Stack: RabbitMQ, FastAPI
Services involved: scheduler
