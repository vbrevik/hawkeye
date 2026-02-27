# Meeting Notes — Meridian Data — 2025-10-16

**Date:** 2025-10-16
**Attendees:** Mohamed Al-Rashid, Alice Chen
**Project:** Meridian Data

## Agenda

- Status update on audit-logger
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Meridian Data. Alice Chen raised concerns about cache invalidation not propagating across regions.
Mohamed Al-Rashid explained that this was related to the recent changes in audit-logger.

We discussed migrating to S3 for better performance. Mohamed Al-Rashid had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between audit-logger and cache-layer was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- Agreed to sunset the legacy Python service by end of Q2.
- We will use Rust for the new service due to memory safety and performance.
- Team agreed on a 2-week sprint cadence going forward.

## Action Items

- [ ] Benchmark the new storage backend — **Alice Chen** — Due 2026-02-23
- [ ] Set up alerting for p99 latency — **Mohamed Al-Rashid** — Due 2026-02-02
- [ ] Implement circuit breakers for downstream calls — **Alice Chen** — Due 2026-01-04
- [ ] Document the deployment process — **Alice Chen** — Due 2026-02-01

## Notes

Stack: S3
Services involved: audit-logger, webhook-handler, cache-layer
