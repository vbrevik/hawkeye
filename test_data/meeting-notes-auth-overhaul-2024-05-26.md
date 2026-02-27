# Meeting Notes — Auth Overhaul — 2024-05-26

**Date:** 2024-05-26
**Attendees:** Elena Rossi, David Park, Sofia Andersen, Clara Johansson
**Project:** Auth Overhaul

## Agenda

- Status update on webhook-handler
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Auth Overhaul. Sofia Andersen raised concerns about goroutine leak in the WebSocket handler.
David Park explained that this was related to the recent changes in webhook-handler.

We discussed migrating to Axum for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between webhook-handler and event-bus was identified as a risk.
Elena Rossi will own this investigation.

## Decisions

- Feature flags will be managed via LaunchDarkly.
- We will use Rust for the new service due to memory safety and performance.
- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Set up alerting for p99 latency — **Clara Johansson** — Due 2026-01-31
- [ ] Document the deployment process — **Elena Rossi** — Due 2026-02-02
- [ ] Add structured logging with trace ids — **Sofia Andersen** — Due 2026-01-26

## Notes

Stack: Axum, Vault
Services involved: webhook-handler, user-service, event-bus
