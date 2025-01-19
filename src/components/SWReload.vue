<script setup lang="ts">
import { computed } from '@vue/reactivity';
import { useRegisterSW } from 'virtual:pwa-register/vue'
import { onMounted, ref, watch } from 'vue';

let timer = ref(0)
const {
	offlineReady,
	needRefresh,
	updateServiceWorker,
} = useRegisterSW()

watch(offlineReady, (newv, _) => {
	if (!newv) return;
	timer.value = 1000
})
onMounted(() => {
	setInterval(() => {
		if (timer.value > 0) timer.value--;
	}, 1)
})
const timerstyle = computed(() => {
	let percent = (timer.value / 1000) * 100
	return `background: linear-gradient(90deg, #777 ${percent}%, #ddd ${percent}%)`
})
async function close() {
	offlineReady.value = false
	needRefresh.value = false
}
</script>

<template>
	<div v-if="(offlineReady && timer > 0) || needRefresh" class="pwa-toast" role="alert">
		<div class="message">
			<span v-if="offlineReady">
				Diese Webseite kann nun offline verwendet werden.
			</span>
			<span v-else>
				New content available, click on reload button to update.
			</span>
		</div>
		<button v-if="needRefresh" @click="updateServiceWorker()">
			Reload
		</button>
		<button @click="close" v-if="needRefresh">
			Close
		</button>
		<span class="timer" :style="timerstyle" v-if="timer > 0 && offlineReady"></span>
	</div>
</template>

<style>
.pwa-toast {
	position: fixed;
	right: 0;
	bottom: 0;
	margin: 16px;
	padding: 12px;
	padding-bottom: 0;
	border: 1px solid #8885;
	border-radius: 4px;
	z-index: 1;
	text-align: left;
	box-shadow: 3px 4px 5px 0 #8885;
	background-color: white;
}

.pwa-toast .message {
	margin-bottom: 8px;
}

.pwa-toast .timer {
	margin-top: 12px;
	margin-bottom: 10px;
	border-radius: 5px;
	height: 0.5rem;
	display: block;
}

.bar {
	display: block;
	margin-bottom: 0;
	height: 0.25rem;
	background-color: red;
}

.pwa-toast button {
	border: 1px solid #8885;
	outline: none;
	margin-right: 5px;
	border-radius: 2px;
	padding: 3px 10px;
}
</style>
