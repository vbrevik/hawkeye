# Meeting Notes — Lighthouse CMS — 2023-05-19

**Date:** 2023-05-19
**Attendees:** David Park, Sofia Andersen, Kofi Mensah
**Project:** Lighthouse CMS

## Agenda

- Status update on event-bus
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. David Park raised concerns about flaky tests in the integration suite.
David Park explained that this was related to the recent changes in event-bus.

We discussed migrating to SQLite for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between event-bus and audit-logger was identified as a risk.
David Park will own this investigation.

## Decisions

- We will use Rust for the new service due to memory safety and performance.

## Action Items

- [ ] Benchmark the new storage backend — **David Park** — Due 2026-02-20
- [ ] Set up alerting for p99 latency — **David Park** — Due 2026-02-26
- [ ] Migrate the legacy monolith to microservices — **David Park** — Due 2026-01-21

## Notes

Stack: SQLite, Rust
Services involved: event-bus, report-generator, audit-logger
