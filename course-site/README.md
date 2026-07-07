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

## Deploying

Set `site` (and, for a project subpath like GitHub Pages, `base`) in
`astro.config.mjs`, then serve `dist/`. Setting `site` also enables the sitemap. On
GitHub Pages, for example:

```js
site: 'https://quarmire.github.io/ndn-course',
base: '/ndn-course',
```
