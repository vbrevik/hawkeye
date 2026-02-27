# Meeting Notes — Pulse Monitoring — 2025-09-27

**Date:** 2025-09-27
**Attendees:** Henrik Larsen, Frank Müller
**Project:** Pulse Monitoring

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Henrik Larsen raised concerns about cache invalidation not propagating across regions.
Frank Müller explained that this was related to the recent changes in search-service.

We discussed migrating to DynamoDB for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and data-warehouse was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Set up alerting for p99 latency — **Henrik Larsen** — Due 2026-01-20
- [ ] Benchmark the new storage backend — **Frank Müller** — Due 2026-01-06
- [ ] Refactor the authentication middleware — **Frank Müller** — Due 2026-01-25

## Notes

Stack: DynamoDB
Services involved: search-service, scheduler, data-warehouse
