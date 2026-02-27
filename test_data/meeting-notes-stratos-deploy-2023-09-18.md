# Meeting Notes — Stratos Deploy — 2023-09-18

**Date:** 2023-09-18
**Attendees:** Elena Rossi, Quinn Murphy, Sofia Andersen, David Park, Gina Torres
**Project:** Stratos Deploy

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Stratos Deploy. Gina Torres raised concerns about token expiry edge case when clock skew > 30s.
Elena Rossi explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Grafana for better performance. Gina Torres had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and api-gateway was identified as a risk.
Quinn Murphy will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Migrate the legacy monolith to microservices — **Elena Rossi** — Due 2026-01-24
- [ ] Benchmark the new storage backend — **Sofia Andersen** — Due 2026-01-24
- [ ] Set up alerting for p99 latency — **Sofia Andersen** — Due 2026-01-19
- [ ] Refactor the authentication middleware — **David Park** — Due 2026-01-02

## Notes

Stack: Grafana
Services involved: webhook-handler, data-warehouse, api-gateway
