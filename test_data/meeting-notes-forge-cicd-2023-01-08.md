# Meeting Notes — Forge CI/CD — 2023-01-08

**Date:** 2023-01-08
**Attendees:** Jae-won Kim, Priya Patel
**Project:** Forge CI/CD

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Forge CI/CD. Priya Patel raised concerns about flaky tests in the integration suite.
Priya Patel explained that this was related to the recent changes in user-service.

We discussed migrating to Docker for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and event-bus was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Priya Patel** — Due 2026-01-12
- [ ] Migrate the legacy monolith to microservices — **Priya Patel** — Due 2026-02-26

## Notes

Stack: Docker
Services involved: user-service, api-gateway, event-bus
