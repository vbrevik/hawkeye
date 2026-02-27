# Meeting Notes — Lighthouse CMS — 2024-10-31

**Date:** 2024-10-31
**Attendees:** Tomas Novak, Gina Torres, Ravi Sharma
**Project:** Lighthouse CMS

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Ravi Sharma raised concerns about retry storm after upstream timeout.
Tomas Novak explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to Elasticsearch for better performance. Gina Torres had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and notification-service was identified as a risk.
Tomas Novak will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- PostgreSQL chosen over MongoDB — relational model fits our query patterns better.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Gina Torres** — Due 2026-01-15
- [ ] Add rate limiting to the public api — **Tomas Novak** — Due 2026-02-12

## Notes

Stack: Elasticsearch
Services involved: analytics-pipeline, notification-service
