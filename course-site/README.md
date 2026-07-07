# course-site

The public face of **ndn-course** — an [Astro](https://astro.build) +
[Starlight](https://starlight.astro.build) site.

- **Multi-page docs** live in `src/content/docs/` (`.md`/`.mdx`, one route per file):
  the splash home (`index.mdx`), the overview + how-it-works pages, a page per phase
  under `modules/`, and the capstones.
- **`public/course-landing.html`** is the standalone decoded-packet landing page (a
  fully self-contained HTML tour), served at `/course-landing.html` and linked from the
  home hero.
- **Site config** (title, sidebar, GitHub link) is in `astro.config.mjs`. The page copy
  is drawn from `docs/syllabus.md`, so the two stay in sync.

## Run it

```sh
npm install
npm run dev      # local dev server at http://localhost:4321
npm run build    # production build to ./dist/
npm run preview  # preview the production build
```

`node_modules/` and `dist/` are gitignored — only the source is committed.

## Deploying to GitHub Pages

`astro.config.mjs` is already set for a GitHub Pages **project site**
(`site: 'https://quarmire.github.io'`, `base: '/ndn-course'`), and internal links are
relative so they survive the base path. The workflow
[`.github/workflows/deploy-site.yml`](../.github/workflows/deploy-site.yml) builds and
publishes the site on every push that touches `course-site/**`.

**One-time setup:** in the repo's **Settings → Pages**, set **Source** to
**GitHub Actions**. Then push (or run the workflow manually) and it deploys to
`https://quarmire.github.io/ndn-course/`.

Deploying to a **user site** or a **custom domain** instead? Drop the `base` line (and
adjust `site`); the relative links still work.
