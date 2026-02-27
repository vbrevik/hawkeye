# Meeting Notes — Apex Security — 2024-07-27

**Date:** 2024-07-27
**Attendees:** Laura Bianchi, Clara Johansson, Sofia Andersen, Frank Müller
**Project:** Apex Security

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Clara Johansson raised concerns about flaky tests in the integration suite.
Sofia Andersen explained that this was related to the recent changes in event-bus.

We discussed migrating to Elasticsearch for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and analytics-pipeline was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Will use Redis for session storage — simple and battle-tested.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Add structured logging with trace ids — **Laura Bianchi** — Due 2026-02-13
- [ ] Migrate the legacy monolith to microservices — **Frank Müller** — Due 2026-02-02
- [ ] Add rate limiting to the public api — **Frank Müller** — Due 2026-01-17

## Notes

Stack: Elasticsearch, gRPC, S3
Services involved: event-bus, auth-service, analytics-pipeline
