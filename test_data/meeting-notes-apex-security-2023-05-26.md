# Meeting Notes — Apex Security — 2023-05-26

**Date:** 2023-05-26
**Attendees:** Quinn Murphy, Nadia Kovač
**Project:** Apex Security

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Nadia Kovač raised concerns about goroutine leak in the WebSocket handler.
Nadia Kovač explained that this was related to the recent changes in report-generator.

We discussed migrating to Redis for better performance. Nadia Kovač had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and api-gateway was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Agreed to sunset the legacy Python service by end of Q2.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Nadia Kovač** — Due 2026-01-12
- [ ] Document the deployment process — **Quinn Murphy** — Due 2026-01-20
- [ ] Refactor the authentication middleware — **Nadia Kovač** — Due 2026-01-12
- [ ] Set up alerting for p99 latency — **Nadia Kovač** — Due 2026-02-26

## Notes

Stack: Redis, Kafka, Elasticsearch
Services involved: report-generator, api-gateway
