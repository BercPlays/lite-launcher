/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {}
	},
	plugins: [require('daisyui')],
	daisyui: {
		base: true, // applies background color and foreground color for root element by default
		styled: true, // include daisyUI colors and design decisions for all components
		utils: true, // adds responsive and modifier utility classes
		themes: [
			{
				liteDark: {
					'color-scheme': 'dark',
					fontFamily:
						'ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,Liberation Mono,Courier New,monospace',
					primary: '#D0D68F',
					secondary: '#818A24',
					accent: '#D3E042',
					neutral: '#7b7f3a', //
					'neutral-content': 'oklch(96.32% 0.007 219.56)', //
					'base-100': '#17180C',
					'--rounded-box': '0',
					'--rounded-btn': '0',
					'--rounded-badge': '0',
					'--tab-radius': '0'
				},
				liteLight: {
					'color-scheme': 'light',
					fontFamily:
						'ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,Liberation Mono,Courier New,monospace',
					primary: '#6A7029',
					secondary: '#D3DB75',
					accent: '#AFBD1F',
					neutral: '#b7bf78',
					'neutral-content': 'oklch(95.63% 0.01 228.89)',
					'base-100': '#F2F3E7',
					'--rounded-box': '0',
					'--rounded-btn': '0',
					'--rounded-badge': '0',
					'--tab-radius': '0'
				}
			}
		]
	}
};
