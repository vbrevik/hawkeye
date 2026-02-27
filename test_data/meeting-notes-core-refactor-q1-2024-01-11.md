# Meeting Notes — Core Refactor Q1 — 2024-01-11

**Date:** 2024-01-11
**Attendees:** Elena Rossi, Alice Chen
**Project:** Core Refactor Q1

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Core Refactor Q1. Alice Chen raised concerns about flaky tests in the integration suite.
Alice Chen explained that this was related to the recent changes in report-generator.

We discussed migrating to React for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and user-service was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Chose gRPC over REST for the internal service mesh.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Set up alerting for p99 latency — **Alice Chen** — Due 2026-01-21
- [ ] Add structured logging with trace ids — **Alice Chen** — Due 2026-01-21
- [ ] Migrate the legacy monolith to microservices — **Elena Rossi** — Due 2026-01-10

## Notes

Stack: React
Services involved: report-generator, search-service, user-service
