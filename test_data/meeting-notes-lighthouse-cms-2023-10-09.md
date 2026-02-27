# Meeting Notes — Lighthouse CMS — 2023-10-09

**Date:** 2023-10-09
**Attendees:** Jae-won Kim, Gina Torres, Elena Rossi, Mohamed Al-Rashid, Sofia Andersen
**Project:** Lighthouse CMS

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Gina Torres raised concerns about retry storm after upstream timeout.
Elena Rossi explained that this was related to the recent changes in scheduler.

We discussed migrating to DynamoDB for better performance. Gina Torres had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and auth-service was identified as a risk.
Gina Torres will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Agreed to sunset the legacy Python service by end of Q2.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Set up alerting for p99 latency — **Sofia Andersen** — Due 2026-01-04
- [ ] Migrate the legacy monolith to microservices — **Gina Torres** — Due 2026-02-21
- [ ] Write runbooks for the on-call team — **Gina Torres** — Due 2026-02-08
- [ ] Benchmark the new storage backend — **Mohamed Al-Rashid** — Due 2026-02-03

## Notes

Stack: DynamoDB, Docker, Axum, Kubernetes
Services involved: scheduler, audit-logger, auth-service
