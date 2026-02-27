# Meeting Notes — Stratos Deploy — 2023-06-20

**Date:** 2023-06-20
**Attendees:** David Park, Ravi Sharma, Gina Torres
**Project:** Stratos Deploy

## Agenda

- Status update on payment-processor
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Gina Torres raised concerns about token expiry edge case when clock skew > 30s.
Ravi Sharma explained that this was related to the recent changes in payment-processor.

We discussed migrating to Kubernetes for better performance. Gina Torres had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between payment-processor and scheduler was identified as a risk.
David Park will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- Will use Redis for session storage — simple and battle-tested.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Review and rotate all secrets in vault — **Ravi Sharma** — Due 2026-02-02
- [ ] Add rate limiting to the public api — **Ravi Sharma** — Due 2026-02-02
- [ ] Add structured logging with trace ids — **Ravi Sharma** — Due 2026-02-11
- [ ] Document the deployment process — **Ravi Sharma** — Due 2026-01-17

## Notes

Stack: Kubernetes, Terraform, Axum, gRPC
Services involved: payment-processor, search-service, scheduler
