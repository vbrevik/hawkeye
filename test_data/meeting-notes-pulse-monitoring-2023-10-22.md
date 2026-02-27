# Meeting Notes — Pulse Monitoring — 2023-10-22

**Date:** 2023-10-22
**Attendees:** Sofia Andersen, David Park
**Project:** Pulse Monitoring

## Agenda

- Status update on audit-logger
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. David Park raised concerns about retry storm after upstream timeout.
Sofia Andersen explained that this was related to the recent changes in audit-logger.

We discussed migrating to Redis for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between audit-logger and notification-service was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Add rate limiting to the public api — **David Park** — Due 2026-02-20
- [ ] Document the deployment process — **Sofia Andersen** — Due 2026-01-27
- [ ] Migrate the legacy monolith to microservices — **Sofia Andersen** — Due 2026-02-01
- [ ] Add structured logging with trace ids — **Sofia Andersen** — Due 2026-01-14

## Notes

Stack: Redis
Services involved: audit-logger, notification-service
