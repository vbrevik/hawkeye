# Meeting Notes — Project Phoenix — 2023-05-17

**Date:** 2023-05-17
**Attendees:** Elena Rossi, Tomas Novak, Priya Patel, Frank Müller, Henrik Larsen
**Project:** Project Phoenix

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Henrik Larsen raised concerns about cache invalidation not propagating across regions.
Tomas Novak explained that this was related to the recent changes in api-gateway.

We discussed migrating to Rust for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and payment-processor was identified as a risk.
Tomas Novak will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Document the deployment process — **Priya Patel** — Due 2026-02-18
- [ ] Implement circuit breakers for downstream calls — **Priya Patel** — Due 2026-01-01
- [ ] Benchmark the new storage backend — **Elena Rossi** — Due 2026-02-03

## Notes

Stack: Rust, Elasticsearch, Kafka
Services involved: api-gateway, payment-processor
