# Meeting Notes — Search Rewrite — 2024-06-24

**Date:** 2024-06-24
**Attendees:** Frank Müller, Alice Chen
**Project:** Search Rewrite

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Search Rewrite. Alice Chen raised concerns about slow query on the user lookup table (missing index).
Frank Müller explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Helm for better performance. Frank Müller had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and webhook-handler was identified as a risk.
Frank Müller will own this investigation.

## Decisions

- Chose gRPC over REST for the internal service mesh.
- Feature flags will be managed via LaunchDarkly.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Frank Müller** — Due 2026-01-20
- [ ] Add structured logging with trace ids — **Alice Chen** — Due 2026-01-29
- [ ] Add rate limiting to the public api — **Alice Chen** — Due 2026-02-17

## Notes

Stack: Helm
Services involved: webhook-handler
