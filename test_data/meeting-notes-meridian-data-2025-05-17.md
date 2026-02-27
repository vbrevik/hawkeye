# Meeting Notes — Meridian Data — 2025-05-17

**Date:** 2025-05-17
**Attendees:** Alice Chen, Laura Bianchi, Jae-won Kim, Frank Müller
**Project:** Meridian Data

## Agenda

- Status update on api-gateway
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Meridian Data. Alice Chen raised concerns about retry storm after upstream timeout.
Laura Bianchi explained that this was related to the recent changes in api-gateway.

We discussed migrating to SQLite for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between api-gateway and api-gateway was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Laura Bianchi** — Due 2026-01-12
- [ ] Document the deployment process — **Laura Bianchi** — Due 2026-02-16
- [ ] Migrate the legacy monolith to microservices — **Frank Müller** — Due 2026-01-03

## Notes

Stack: SQLite, Kafka, Vault
Services involved: api-gateway
