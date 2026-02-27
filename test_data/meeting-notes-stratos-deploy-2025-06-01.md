# Meeting Notes — Stratos Deploy — 2025-06-01

**Date:** 2025-06-01
**Attendees:** Clara Johansson, Quinn Murphy, Sofia Andersen
**Project:** Stratos Deploy

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Quinn Murphy raised concerns about token expiry edge case when clock skew > 30s.
Quinn Murphy explained that this was related to the recent changes in scheduler.

We discussed migrating to React for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Clara Johansson will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Add structured logging with trace ids — **Sofia Andersen** — Due 2026-02-21
- [ ] Refactor the authentication middleware — **Quinn Murphy** — Due 2026-01-07

## Notes

Stack: React, Nginx, Elasticsearch
Services involved: scheduler
