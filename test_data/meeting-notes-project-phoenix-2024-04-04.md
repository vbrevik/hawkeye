# Meeting Notes — Project Phoenix — 2024-04-04

**Date:** 2024-04-04
**Attendees:** David Park, Elena Rossi, Mohamed Al-Rashid
**Project:** Project Phoenix

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Elena Rossi raised concerns about cache invalidation not propagating across regions.
David Park explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Kubernetes for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and search-service was identified as a risk.
David Park will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.
- Team agreed on a 2-week sprint cadence going forward.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Add rate limiting to the public api — **David Park** — Due 2026-02-06
- [ ] Document the deployment process — **Elena Rossi** — Due 2026-01-10

## Notes

Stack: Kubernetes, Rust
Services involved: webhook-handler, search-service
