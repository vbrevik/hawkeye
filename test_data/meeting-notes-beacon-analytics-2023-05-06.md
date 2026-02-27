# Meeting Notes — Beacon Analytics — 2023-05-06

**Date:** 2023-05-06
**Attendees:** Sofia Andersen, Bob Martins
**Project:** Beacon Analytics

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Beacon Analytics. Bob Martins raised concerns about goroutine leak in the WebSocket handler.
Bob Martins explained that this was related to the recent changes in scheduler.

We discussed migrating to Prometheus for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and analytics-pipeline was identified as a risk.
Bob Martins will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Refactor the authentication middleware — **Bob Martins** — Due 2026-02-21
- [ ] Review and rotate all secrets in vault — **Sofia Andersen** — Due 2026-01-01
- [ ] Write runbooks for the on-call team — **Bob Martins** — Due 2026-02-25
- [ ] Implement circuit breakers for downstream calls — **Bob Martins** — Due 2026-02-16

## Notes

Stack: Prometheus, Grafana, RabbitMQ, React
Services involved: scheduler, analytics-pipeline
