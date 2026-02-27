# Meeting Notes — Nexus API — 2023-01-06

**Date:** 2023-01-06
**Attendees:** Jae-won Kim, Priya Patel, David Park, Tomas Novak
**Project:** Nexus API

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. David Park raised concerns about flaky tests in the integration suite.
Jae-won Kim explained that this was related to the recent changes in event-bus.

We discussed migrating to Helm for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and api-gateway was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **David Park** — Due 2026-01-11
- [ ] Refactor the authentication middleware — **Jae-won Kim** — Due 2026-01-10
- [ ] Add rate limiting to the public api — **David Park** — Due 2026-01-30
- [ ] Review and rotate all secrets in vault — **Priya Patel** — Due 2026-01-23

## Notes

Stack: Helm, Nginx, gRPC
Services involved: event-bus, api-gateway
