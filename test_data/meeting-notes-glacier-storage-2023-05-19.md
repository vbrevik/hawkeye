# Meeting Notes — Glacier Storage — 2023-05-19

**Date:** 2023-05-19
**Attendees:** Tomas Novak, Laura Bianchi, Oscar Lindberg, Elena Rossi, Ravi Sharma
**Project:** Glacier Storage

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Laura Bianchi raised concerns about memory leak in the worker pool.
Elena Rossi explained that this was related to the recent changes in auth-service.

We discussed migrating to Go for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and analytics-pipeline was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- We will require code review from 2 engineers before merging.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Set up alerting for p99 latency — **Oscar Lindberg** — Due 2026-02-02
- [ ] Refactor the authentication middleware — **Tomas Novak** — Due 2026-01-15

## Notes

Stack: Go, Grafana, GraphQL
Services involved: auth-service, notification-service, analytics-pipeline
