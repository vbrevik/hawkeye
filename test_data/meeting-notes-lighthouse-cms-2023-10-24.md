# Meeting Notes — Lighthouse CMS — 2023-10-24

**Date:** 2023-10-24
**Attendees:** Alice Chen, Elena Rossi, Oscar Lindberg, Sofia Andersen
**Project:** Lighthouse CMS

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Elena Rossi raised concerns about slow query on the user lookup table (missing index).
Sofia Andersen explained that this was related to the recent changes in media-uploader.

We discussed migrating to DynamoDB for better performance. Elena Rossi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and media-uploader was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Review and rotate all secrets in vault — **Elena Rossi** — Due 2026-01-23
- [ ] Write runbooks for the on-call team — **Elena Rossi** — Due 2026-01-13
- [ ] Refactor the authentication middleware — **Alice Chen** — Due 2026-01-24
- [ ] Implement circuit breakers for downstream calls — **Oscar Lindberg** — Due 2026-01-02

## Notes

Stack: DynamoDB, Helm, Kafka, PostgreSQL
Services involved: media-uploader
