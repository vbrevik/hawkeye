# Meeting Notes — Atlas Platform — 2024-05-12

**Date:** 2024-05-12
**Attendees:** Tomas Novak, David Park, Bob Martins, Laura Bianchi
**Project:** Atlas Platform

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. David Park raised concerns about goroutine leak in the WebSocket handler.
David Park explained that this was related to the recent changes in api-gateway.

We discussed migrating to DynamoDB for better performance. Bob Martins had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and data-warehouse was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Chose gRPC over REST for the internal service mesh.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Set up alerting for p99 latency — **Laura Bianchi** — Due 2026-01-28
- [ ] Refactor the authentication middleware — **Bob Martins** — Due 2026-01-21
- [ ] Review and rotate all secrets in vault — **David Park** — Due 2026-02-23
- [ ] Add structured logging with trace ids — **Tomas Novak** — Due 2026-01-18

## Notes

Stack: DynamoDB, GraphQL, Celery
Services involved: api-gateway, data-warehouse
