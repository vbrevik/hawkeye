# Meeting Notes — Meridian Data — 2025-01-09

**Date:** 2025-01-09
**Attendees:** Alice Chen, Sofia Andersen, Jae-won Kim
**Project:** Meridian Data

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Meridian Data. Alice Chen raised concerns about token expiry edge case when clock skew > 30s.
Sofia Andersen explained that this was related to the recent changes in media-uploader.

We discussed migrating to DynamoDB for better performance. Jae-won Kim had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and media-uploader was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Benchmark the new storage backend — **Jae-won Kim** — Due 2026-02-22
- [ ] Refactor the authentication middleware — **Alice Chen** — Due 2026-02-25
- [ ] Write runbooks for the on-call team — **Jae-won Kim** — Due 2026-01-31

## Notes

Stack: DynamoDB, Nginx, Elasticsearch
Services involved: media-uploader
