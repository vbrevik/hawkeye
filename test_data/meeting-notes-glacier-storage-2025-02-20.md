# Meeting Notes — Glacier Storage — 2025-02-20

**Date:** 2025-02-20
**Attendees:** Jae-won Kim, Alice Chen
**Project:** Glacier Storage

## Agenda

- Status update on audit-logger
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Alice Chen raised concerns about goroutine leak in the WebSocket handler.
Alice Chen explained that this was related to the recent changes in audit-logger.

We discussed migrating to PostgreSQL for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between audit-logger and webhook-handler was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Review and rotate all secrets in vault — **Alice Chen** — Due 2026-02-14
- [ ] Implement circuit breakers for downstream calls — **Jae-won Kim** — Due 2026-01-27

## Notes

Stack: PostgreSQL, Rust, S3, React
Services involved: audit-logger, webhook-handler
