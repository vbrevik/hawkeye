# Meeting Notes — Lighthouse CMS — 2024-02-21

**Date:** 2024-02-21
**Attendees:** Oscar Lindberg, Laura Bianchi
**Project:** Lighthouse CMS

## Agenda

- Status update on search-service
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Lighthouse CMS. Oscar Lindberg raised concerns about flaky tests in the integration suite.
Oscar Lindberg explained that this was related to the recent changes in search-service.

We discussed migrating to TypeScript for better performance. Laura Bianchi had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between search-service and auth-service was identified as a risk.
Laura Bianchi will own this investigation.

## Decisions

- We will require code review from 2 engineers before merging.

## Action Items

- [ ] Implement circuit breakers for downstream calls — **Oscar Lindberg** — Due 2026-01-14
- [ ] Refactor the authentication middleware — **Oscar Lindberg** — Due 2026-02-17
- [ ] Write runbooks for the on-call team — **Oscar Lindberg** — Due 2026-02-17
- [ ] Document the deployment process — **Laura Bianchi** — Due 2026-01-16

## Notes

Stack: TypeScript
Services involved: search-service, audit-logger, auth-service
