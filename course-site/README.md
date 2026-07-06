# course-site/

The public face of the course.

- `course-landing.html` — the interim front door: a fully self-contained,
  decoded-packet landing page (open it directly in a browser). Its copy is drawn
  from `docs/syllabus.md` §§1/5/7, so the two stay in sync.
- When Phase 1 content is real, this directory becomes an Astro Starlight
  project with the landing page as its index and the module text as MDX.
  mdBook is at most an optional export target — not the primary surface.
- A Dioxus port of the site is a curated Capstone-B ticket.
