# Meeting Notes — Core Refactor Q1 — 2023-10-27

**Date:** 2023-10-27
**Attendees:** Laura Bianchi, Bob Martins, Henrik Larsen, Priya Patel, Ravi Sharma
**Project:** Core Refactor Q1

## Agenda

- Status update on audit-logger
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Henrik Larsen raised concerns about race condition during concurrent writes.
Ravi Sharma explained that this was related to the recent changes in audit-logger.

We discussed migrating to Nginx for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between audit-logger and analytics-pipeline was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Ravi Sharma** — Due 2026-02-04
- [ ] Refactor the authentication middleware — **Ravi Sharma** — Due 2026-01-12
- [ ] Document the deployment process — **Priya Patel** — Due 2026-02-11

## Notes

Stack: Nginx, GraphQL, PostgreSQL, DynamoDB
Services involved: audit-logger, scheduler, analytics-pipeline
