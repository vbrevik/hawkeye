# Meeting Notes — Lighthouse CMS — 2023-04-25

**Date:** 2023-04-25
**Attendees:** Mohamed Al-Rashid, David Park
**Project:** Lighthouse CMS

## Agenda

- Status update on auth-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Mohamed Al-Rashid raised concerns about flaky tests in the integration suite.
Mohamed Al-Rashid explained that this was related to the recent changes in auth-service.

We discussed migrating to Nginx for better performance. Mohamed Al-Rashid had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between auth-service and auth-service was identified as a risk.
Mohamed Al-Rashid will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Benchmark the new storage backend — **Mohamed Al-Rashid** — Due 2026-01-12
- [ ] Migrate the legacy monolith to microservices — **Mohamed Al-Rashid** — Due 2026-02-27
- [ ] Implement circuit breakers for downstream calls — **David Park** — Due 2026-02-22

## Notes

Stack: Nginx, DynamoDB
Services involved: auth-service
