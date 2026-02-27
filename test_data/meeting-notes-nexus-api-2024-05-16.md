# Meeting Notes — Nexus API — 2024-05-16

**Date:** 2024-05-16
**Attendees:** Alice Chen, Priya Patel, Frank Müller, Laura Bianchi
**Project:** Nexus API

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Nexus API. Laura Bianchi raised concerns about cache invalidation not propagating across regions.
Frank Müller explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Axum for better performance. Priya Patel had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and event-bus was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Review and rotate all secrets in vault — **Frank Müller** — Due 2026-02-27
- [ ] Implement circuit breakers for downstream calls — **Frank Müller** — Due 2026-02-22
- [ ] Add structured logging with trace ids — **Laura Bianchi** — Due 2026-02-08
- [ ] Set up alerting for p99 latency — **Laura Bianchi** — Due 2026-02-04

## Notes

Stack: Axum, Kafka
Services involved: webhook-handler, event-bus
