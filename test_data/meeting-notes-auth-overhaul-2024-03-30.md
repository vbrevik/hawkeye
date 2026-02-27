# Meeting Notes — Auth Overhaul — 2024-03-30

**Date:** 2024-03-30
**Attendees:** Laura Bianchi, Alice Chen, Nadia Kovač, Bob Martins, Mohamed Al-Rashid
**Project:** Auth Overhaul

## Agenda

- Status update on analytics-pipeline
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Nadia Kovač raised concerns about cache invalidation not propagating across regions.
Alice Chen explained that this was related to the recent changes in analytics-pipeline.

We discussed migrating to Celery for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between analytics-pipeline and analytics-pipeline was identified as a risk.
Nadia Kovač will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- Agreed to sunset the legacy Python service by end of Q2.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Set up alerting for p99 latency — **Bob Martins** — Due 2026-02-14
- [ ] Implement circuit breakers for downstream calls — **Alice Chen** — Due 2026-02-18

## Notes

Stack: Celery
Services involved: analytics-pipeline
