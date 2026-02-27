# Meeting Notes — Apex Security — 2025-03-28

**Date:** 2025-03-28
**Attendees:** Quinn Murphy, Kofi Mensah, Priya Patel
**Project:** Apex Security

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Quinn Murphy raised concerns about goroutine leak in the WebSocket handler.
Quinn Murphy explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Elasticsearch for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and scheduler was identified as a risk.
Priya Patel will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Add rate limiting to the public api — **Priya Patel** — Due 2026-01-28
- [ ] Write runbooks for the on-call team — **Kofi Mensah** — Due 2026-01-25
- [ ] Add structured logging with trace ids — **Priya Patel** — Due 2026-01-12
- [ ] Migrate the legacy monolith to microservices — **Priya Patel** — Due 2026-02-06

## Notes

Stack: Elasticsearch, S3, SQLite
Services involved: webhook-handler, report-generator, scheduler
