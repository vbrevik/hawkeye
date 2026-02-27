# Meeting Notes — Atlas Platform — 2025-12-09

**Date:** 2025-12-09
**Attendees:** Bob Martins, Laura Bianchi, Quinn Murphy, Alice Chen, Frank Müller
**Project:** Atlas Platform

## Agenda

- Status update on data-warehouse
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Atlas Platform. Laura Bianchi raised concerns about cache invalidation not propagating across regions.
Laura Bianchi explained that this was related to the recent changes in data-warehouse.

We discussed migrating to Elasticsearch for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between data-warehouse and report-generator was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Adopted conventional commits across all repositories.

## Action Items

- [ ] Add rate limiting to the public api — **Alice Chen** — Due 2026-01-11
- [ ] Migrate the legacy monolith to microservices — **Frank Müller** — Due 2026-02-06

## Notes

Stack: Elasticsearch
Services involved: data-warehouse, event-bus, report-generator
