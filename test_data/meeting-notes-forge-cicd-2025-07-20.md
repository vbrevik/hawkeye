# Meeting Notes — Forge CI/CD — 2025-07-20

**Date:** 2025-07-20
**Attendees:** Oscar Lindberg, Jae-won Kim
**Project:** Forge CI/CD

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Forge CI/CD. Oscar Lindberg raised concerns about SSL certificate not renewing automatically.
Jae-won Kim explained that this was related to the recent changes in media-uploader.

We discussed migrating to Prometheus for better performance. Oscar Lindberg had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and user-service was identified as a risk.
Jae-won Kim will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Add structured logging with trace ids — **Jae-won Kim** — Due 2026-01-27
- [ ] Set up alerting for p99 latency — **Oscar Lindberg** — Due 2026-01-29
- [ ] Refactor the authentication middleware — **Jae-won Kim** — Due 2026-01-20

## Notes

Stack: Prometheus, RabbitMQ, Docker, Axum
Services involved: media-uploader, analytics-pipeline, user-service
