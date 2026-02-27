# Meeting Notes — Lighthouse CMS — 2025-01-20

**Date:** 2025-01-20
**Attendees:** Kofi Mensah, Mohamed Al-Rashid
**Project:** Lighthouse CMS

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Mohamed Al-Rashid raised concerns about disk I/O bottleneck during bulk import.
Kofi Mensah explained that this was related to the recent changes in media-uploader.

We discussed migrating to React for better performance. Kofi Mensah had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and media-uploader was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Set up alerting for p99 latency — **Kofi Mensah** — Due 2026-01-22
- [ ] Review and rotate all secrets in vault — **Kofi Mensah** — Due 2026-01-02
- [ ] Document the deployment process — **Mohamed Al-Rashid** — Due 2026-01-03

## Notes

Stack: React, DynamoDB
Services involved: media-uploader
