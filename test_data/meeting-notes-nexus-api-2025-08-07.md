# Meeting Notes — Nexus API — 2025-08-07

**Date:** 2025-08-07
**Attendees:** Nadia Kovač, Alice Chen, David Park, Frank Müller, Elena Rossi
**Project:** Nexus API

## Agenda

- Status update on user-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Frank Müller raised concerns about goroutine leak in the WebSocket handler.
Frank Müller explained that this was related to the recent changes in user-service.

We discussed migrating to PostgreSQL for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between user-service and user-service was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Adopted conventional commits across all repositories.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Add structured logging with trace ids — **Elena Rossi** — Due 2026-01-19
- [ ] Implement circuit breakers for downstream calls — **Nadia Kovač** — Due 2026-02-01

## Notes

Stack: PostgreSQL, Prometheus, Kubernetes, Kafka
Services involved: user-service
