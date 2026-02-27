# Meeting Notes — Lighthouse CMS — 2023-05-12

**Date:** 2023-05-12
**Attendees:** Sofia Andersen, Ravi Sharma
**Project:** Lighthouse CMS

## Agenda

- Status update on media-uploader
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Ravi Sharma raised concerns about race condition during concurrent writes.
Sofia Andersen explained that this was related to the recent changes in media-uploader.

We discussed migrating to PostgreSQL for better performance. Ravi Sharma had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between media-uploader and payment-processor was identified as a risk.
Ravi Sharma will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Refactor the authentication middleware — **Sofia Andersen** — Due 2026-02-04
- [ ] Add rate limiting to the public api — **Sofia Andersen** — Due 2026-02-16

## Notes

Stack: PostgreSQL
Services involved: media-uploader, auth-service, payment-processor
