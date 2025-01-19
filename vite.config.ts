import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import topLevelAwait from 'vite-plugin-top-level-await'
import { VitePWA } from 'vite-plugin-pwa'

// https://vite.dev/config/
export default defineConfig({
	plugins: [vue(), topLevelAwait(), VitePWA({
		registerType: "prompt",
		injectRegister: "auto",
		manifest: {
			name: "Stroboskopbilder",
			short_name: "Strobo",
			description: "Erstellt Stroboskopbilder aus Videos oder mit der Kamera",
			theme_color: "dodgerblue",
			display: "fullscreen",
			icons: [
				{
					src: 'pwa-100-100.png',
					sizes: '100x100',
					type: 'image/png'
				},
				{
					src: "pwa-512.png",
					sizes: "512x512",
					type: 'image/png'
				}
			],
			screenshots: [
				{
					src: "scr-mobile.png",
					sizes: "1440x3200",
					type: "image/png",
					form_factor: "narrow",
					label: "Homescreen"
				},
				{
					src: "scr-wide.png",
					sizes: "1920x1080",
					type: "image/png",
					form_factor: "wide",
					label: "Homescreen"
				}
			]
		},
		devOptions: {
			enabled: false
		},
		workbox: {
			globPatterns: ["**/*.{js,html,png,css}"]
		}
	})],
})
