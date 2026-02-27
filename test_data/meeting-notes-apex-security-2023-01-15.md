# Meeting Notes — Apex Security — 2023-01-15

**Date:** 2023-01-15
**Attendees:** Quinn Murphy, Alice Chen
**Project:** Apex Security

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Alice Chen raised concerns about disk I/O bottleneck during bulk import.
Alice Chen explained that this was related to the recent changes in media-uploader.

We discussed migrating to Terraform for better performance. Alice Chen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and search-service was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- Chose gRPC over REST for the internal service mesh.

## Action Items

- [ ] Set up alerting for p99 latency — **Alice Chen** — Due 2026-02-05
- [ ] Add rate limiting to the public api — **Alice Chen** — Due 2026-01-12
- [ ] Migrate the legacy monolith to microservices — **Alice Chen** — Due 2026-02-05

## Notes

Stack: Terraform, ArgoCD, gRPC
Services involved: media-uploader, search-service
