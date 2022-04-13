const colors = require('tailwindcss/colors');

const config = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	darkMode: 'class',
	plugins: [],
	theme: {
		fontSize: {
      '2xl': ['40px', {
        letterSpacing: '.3em',
				lineHeight: '40px',
      }],
    }
	}
};

module.exports = config;
