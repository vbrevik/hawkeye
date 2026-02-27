# Meeting Notes — Nexus API — 2026-02-24

**Date:** 2026-02-24
**Attendees:** Frank Müller, Nadia Kovač, David Park
**Project:** Nexus API

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Frank Müller raised concerns about cache invalidation not propagating across regions.
Nadia Kovač explained that this was related to the recent changes in api-gateway.

We discussed migrating to GraphQL for better performance. Nadia Kovač had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and api-gateway was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.
- Team agreed on a 2-week sprint cadence going forward.
- Decided to go with a pull-based deployment model using ArgoCD.

## Action Items

- [ ] Benchmark the new storage backend — **Nadia Kovač** — Due 2026-01-11
- [ ] Add structured logging with trace ids — **Frank Müller** — Due 2026-02-07
- [ ] Write runbooks for the on-call team — **David Park** — Due 2026-02-22
- [ ] Set up alerting for p99 latency — **Nadia Kovač** — Due 2026-01-29

## Notes

Stack: GraphQL, Kafka
Services involved: api-gateway
