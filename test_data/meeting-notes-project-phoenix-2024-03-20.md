# Meeting Notes — Project Phoenix — 2024-03-20

**Date:** 2024-03-20
**Attendees:** Elena Rossi, Henrik Larsen, Clara Johansson, Tomas Novak
**Project:** Project Phoenix

## Agenda

- Status update on cache-layer
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. Tomas Novak raised concerns about slow query on the user lookup table (missing index).
Clara Johansson explained that this was related to the recent changes in cache-layer.

We discussed migrating to FastAPI for better performance. Henrik Larsen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between cache-layer and user-service was identified as a risk.
Henrik Larsen will own this investigation.

## Decisions

- Will use Redis for session storage — simple and battle-tested.
- Feature flags will be managed via LaunchDarkly.

## Action Items

- [ ] Add rate limiting to the public api — **Clara Johansson** — Due 2026-01-26
- [ ] Set up alerting for p99 latency — **Elena Rossi** — Due 2026-02-07

## Notes

Stack: FastAPI, SQLite, Nginx, gRPC
Services involved: cache-layer, media-uploader, user-service
