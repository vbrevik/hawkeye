# Meeting Notes — Glacier Storage — 2025-09-21

**Date:** 2025-09-21
**Attendees:** Bob Martins, Sofia Andersen
**Project:** Glacier Storage

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Glacier Storage. Bob Martins raised concerns about flaky tests in the integration suite.
Bob Martins explained that this was related to the recent changes in report-generator.

We discussed migrating to DynamoDB for better performance. Sofia Andersen had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and auth-service was identified as a risk.
Sofia Andersen will own this investigation.

## Decisions

- Team agreed on a 2-week sprint cadence going forward.
- Feature flags will be managed via LaunchDarkly.
- Will use Redis for session storage — simple and battle-tested.

## Action Items

- [ ] Set up alerting for p99 latency — **Sofia Andersen** — Due 2026-02-20
- [ ] Implement circuit breakers for downstream calls — **Bob Martins** — Due 2026-02-22

## Notes

Stack: DynamoDB, Nginx
Services involved: report-generator, api-gateway, auth-service
