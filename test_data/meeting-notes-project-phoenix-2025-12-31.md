# Meeting Notes — Project Phoenix — 2025-12-31

**Date:** 2025-12-31
**Attendees:** David Park, Jae-won Kim, Isabelle Dupont
**Project:** Project Phoenix

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Jae-won Kim raised concerns about flaky tests in the integration suite.
Jae-won Kim explained that this was related to the recent changes in event-bus.

We discussed migrating to Prometheus for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and event-bus was identified as a risk.
Isabelle Dupont will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Refactor the authentication middleware — **David Park** — Due 2026-01-22
- [ ] Implement circuit breakers for downstream calls — **David Park** — Due 2026-02-23
- [ ] Add rate limiting to the public api — **David Park** — Due 2026-01-04
- [ ] Add structured logging with trace ids — **Jae-won Kim** — Due 2026-02-19

## Notes

Stack: Prometheus, gRPC
Services involved: event-bus
