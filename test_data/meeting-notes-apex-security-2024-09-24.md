# Meeting Notes — Apex Security — 2024-09-24

**Date:** 2024-09-24
**Attendees:** Jae-won Kim, Laura Bianchi, Alice Chen, Quinn Murphy
**Project:** Apex Security

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Apex Security. Alice Chen raised concerns about slow query on the user lookup table (missing index).
Quinn Murphy explained that this was related to the recent changes in payment-processor.

We discussed migrating to SQLite for better performance. Quinn Murphy had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and payment-processor was identified as a risk.
Alice Chen will own this investigation.

## Decisions

- Decided to go with a pull-based deployment model using ArgoCD.
- We will use Rust for the new service due to memory safety and performance.
- Agreed to sunset the legacy Python service by end of Q2.

## Action Items

- [ ] Benchmark the new storage backend — **Alice Chen** — Due 2026-02-09
- [ ] Refactor the authentication middleware — **Alice Chen** — Due 2026-01-27
- [ ] Review and rotate all secrets in vault — **Alice Chen** — Due 2026-01-11
- [ ] Migrate the legacy monolith to microservices — **Jae-won Kim** — Due 2026-01-30

## Notes

Stack: SQLite, Redis
Services involved: payment-processor
