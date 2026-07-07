// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	// GitHub Pages project site: served at https://<user>.github.io/<repo>/.
	// Change these if you deploy to a user site or a custom domain (drop `base`).
	site: 'https://quarmire.github.io',
	base: '/ndn-course',
	integrations: [
		starlight({
			title: 'ndn-course',
			description:
				'Learn Rust where it is real — an applied course taught inside a working named-data networking stack.',
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/Quarmire/ndn-course',
				},
			],
			sidebar: [
				{
					label: 'Start here',
					items: [
						{ label: 'Overview', slug: 'getting-started' },
						{ label: 'How the course works', slug: 'the-course' },
					],
				},
				{
					label: 'The journey',
					items: [
						{ label: 'Phase 0 — Orientation', slug: 'modules/phase-0-orientation' },
						{ label: 'Phase 1 — Bytes, memory, bugs', slug: 'modules/phase-1-bytes' },
						{ label: 'Phase 2 — Types that carry meaning', slug: 'modules/phase-2-types' },
						{ label: 'Phase 3 — Threads, processes, async', slug: 'modules/phase-3-concurrency' },
						{ label: 'Phase 4 — Architecture & NDF', slug: 'modules/phase-4-architecture' },
					],
				},
				{ label: 'Capstones', slug: 'capstones' },
			],
		}),
	],
});
