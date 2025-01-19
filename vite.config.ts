import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import { VitePWA } from 'vite-plugin-pwa'

// https://vite.dev/config/
export default defineConfig({
	plugins: [vue(), wasm(), topLevelAwait(), VitePWA({
		registerType: "prompt",
		injectRegister: "auto",
		devOptions: {
			enabled: false
		},
		workbox: {
			globPatterns: ["**/*.{js,html,wasm,css}"]
		}
	})],
})
