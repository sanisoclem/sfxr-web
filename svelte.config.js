import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-cloudflare';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [
		preprocess({
			postcss: true
		})
	],

	kit: {
		adapter: adapter(),
		vite: {
			optimizeDeps: {
				exclude: ['marked']
			},
			server: {
				fs: {
					allow: [
						"sfxr-rs/pkg"
					]
				}
			}
		}
	}
};

export default config;
