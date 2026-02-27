# Meeting Notes — Pulse Monitoring — 2023-12-25

**Date:** 2023-12-25
**Attendees:** Ravi Sharma, Priya Patel, Bob Martins, Laura Bianchi, Mohamed Al-Rashid
**Project:** Pulse Monitoring

## Agenda

- Status update on scheduler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Pulse Monitoring. Laura Bianchi raised concerns about memory leak in the worker pool.
Mohamed Al-Rashid explained that this was related to the recent changes in scheduler.

We discussed migrating to React for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between scheduler and scheduler was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- Team agreed on a 2-week sprint cadence going forward.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Write runbooks for the on-call team — **Mohamed Al-Rashid** — Due 2026-02-03
- [ ] Review and rotate all secrets in vault — **Priya Patel** — Due 2026-01-25

## Notes

Stack: React, Docker, Celery, Rust
Services involved: scheduler
