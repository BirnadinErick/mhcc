const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				red: '#FD6E86',
				yellow: '#F9F785',
				green: '#2EDCA1',
				indigo: '#00171F'
			},
			fontFamily: {
				sans: ['Noto Sans', ...defaultTheme.fontFamily.sans],
				serif: ['DM Serif Display', ...defaultTheme.fontFamily.serif],
				mono: ['Jetbrains Mono', ...defaultTheme.fontFamily.mono]
			}
		}
	},
	plugins: []
};
