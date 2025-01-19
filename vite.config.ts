import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import topLevelAwait from 'vite-plugin-top-level-await'
import { VitePWA } from 'vite-plugin-pwa'

import * as child from "child_process";

const git_version = child.execSync("git describe --long --dirty --tags --always").toString().replaceAll("\n", "");

// https://vite.dev/config/
export default defineConfig({
	define: {
		GIT_VERSION: JSON.stringify(git_version)
	},
	plugins: [vue(), topLevelAwait(), VitePWA({
		registerType: "autoUpdate",
		injectRegister: "inline",
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
			globPatterns: ["**/*.{js,html,png,css}"],
			runtimeCaching: [
				{
					urlPattern: "*/*",
					handler: "NetworkFirst",
					options: {
						cacheName: "app",
						expiration: {
							maxEntries: 1000,
							maxAgeSeconds: 31 * 24 * 60 * 60, // month
						},
						precacheFallback: {
							fallbackURL: "/index.html"
						},
					},
				}
			]

		}
	})],
})
