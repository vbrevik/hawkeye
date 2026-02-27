# Meeting Notes — Project Phoenix — 2023-07-08

**Date:** 2023-07-08
**Attendees:** Tomas Novak, David Park, Sofia Andersen
**Project:** Project Phoenix

## Agenda

- Status update on report-generator
- Review open issues
- Action items from last week

## Discussion

The team reviewed the current state of Project Phoenix. David Park raised concerns about race condition during concurrent writes.
Sofia Andersen explained that this was related to the recent changes in report-generator.

We discussed migrating to Terraform for better performance. David Park had reservations
about the migration timeline but agreed it was the right long-term direction.

The integration between report-generator and report-generator was identified as a risk.
David Park will own this investigation.

## Decisions

- Adopted conventional commits across all repositories.

## Action Items

- [ ] Review and rotate all secrets in vault — **David Park** — Due 2026-02-14
- [ ] Migrate the legacy monolith to microservices — **Sofia Andersen** — Due 2026-02-19

## Notes

Stack: Terraform, Docker
Services involved: report-generator
