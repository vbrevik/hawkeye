# Meeting Notes — Glacier Storage — 2024-06-13

**Date:** 2024-06-13
**Attendees:** Oscar Lindberg, Laura Bianchi, Mohamed Al-Rashid
**Project:** Glacier Storage

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Laura Bianchi raised concerns about token expiry edge case when clock skew > 30s.
Laura Bianchi explained that this was related to the recent changes in api-gateway.

We discussed migrating to Redis for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and user-service was identified as a risk.
Oscar Lindberg will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Set up alerting for p99 latency — **Mohamed Al-Rashid** — Due 2026-01-31
- [ ] Refactor the authentication middleware — **Laura Bianchi** — Due 2026-01-31

## Notes

Stack: Redis, Kafka, FastAPI, Rust
Services involved: api-gateway, user-service
