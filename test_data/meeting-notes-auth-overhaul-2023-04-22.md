# Meeting Notes — Auth Overhaul — 2023-04-22

**Date:** 2023-04-22
**Attendees:** Ravi Sharma, Clara Johansson, Jae-won Kim, Sofia Andersen, Isabelle Dupont
**Project:** Auth Overhaul

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Isabelle Dupont raised concerns about race condition during concurrent writes.
Clara Johansson explained that this was related to the recent changes in media-uploader.

We discussed migrating to Kafka for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and media-uploader was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Document the deployment process — **Sofia Andersen** — Due 2026-01-22
- [ ] Add structured logging with trace ids — **Isabelle Dupont** — Due 2026-02-12
- [ ] Set up alerting for p99 latency — **Jae-won Kim** — Due 2026-02-14
- [ ] Migrate the legacy monolith to microservices — **Isabelle Dupont** — Due 2026-01-10

## Notes

Stack: Kafka, Docker
Services involved: media-uploader
