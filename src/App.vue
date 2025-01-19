<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import { init, read } from '../wasm/pkg'
import { onMounted, ref, useTemplateRef } from 'vue';
import SWReload from './components/SWReload.vue';
onMounted(() => {
	init()
})
function fileadd() {
	if (!inputelem.value || !inputelem.value.files || !inputelem.value.files[0]) return
	let url = URL.createObjectURL(inputelem.value.files[0])
	console.log(url)
	videourl.value = url

}

let videourl = ref("");
let threshold = ref(40);
let frameinterval = ref(10);
let videoelem = useTemplateRef("videoelem")
let inputelem = useTemplateRef("inputelem")
let imgelem = useTemplateRef("targetimg")
let counter = 0; // no ref
let canvas = document.createElement("canvas")
let twod = canvas.getContext("2d", { alpha: false, willReadFrequently: true })
let dimensions = { width: 0, height: 0 }
async function canplay() {
	if (!videoelem.value) return
	videoelem.value.style.display = ""
	let height = videoelem.value?.videoHeight ? videoelem.value?.videoHeight : 0;
	let width = videoelem.value?.videoWidth ? videoelem.value?.videoWidth : 0;
	canvas.height = height;
	canvas.width = width;
	dimensions = { width, height }
	videoelem.value.volume = 0
	videoelem.value.playbackRate = 0.1
	videoelem.value.loop = false
	await videoelem.value.play()
	videoelem.value.pause()
}
async function start() {
	if (!videoelem.value) return
	images = []
	let vid = videoelem.value
	if (imgelem.value) imgelem.value.src = ""
	vid.currentTime = 0
	let frametime = 1 / frameinterval.value
	while (vid.currentTime + frametime + 0.05 < vid.duration) {
		console.log(`capture ${vid.currentTime}`)
		vid.currentTime += frametime
		await vid.play()
		vid.pause()
		await delay((frametime * 1000))
		let img = await capture()
		if(img) images.push(img)
		await delay(5)
	}
	videoend()
}
let rtiv = 0;
let rtcurrent = new Uint8Array();
async function startRt() {
	if (!videoelem.value) return;
	let cam = await navigator.mediaDevices.getUserMedia({ video: true })
	videoelem.value.srcObject = cam
	let first = await capture();
	if(!first) return;
	rtcurrent = first;
	rtiv = setInterval(async () => {
		let img = await capture()
		if (!img) return;
		

	}, (1 / frameinterval.value) * 1000)
}
//function stopRt() {
//	clearInterval(rtiv);
//}


function delay(time: number) {
	return new Promise((resolve) => setTimeout(resolve, time))
}
let images: Uint8Array[] = [];
async function capture() {
	if (!videoelem.value || !twod) return;
	if (counter % frameinterval.value != 0) return;
	let { width, height } = dimensions
	twod.clearRect(0, 0, width, height)
	twod.drawImage(videoelem.value, 0, 0, width, height)
	let idata = twod.getImageData(0, 0, width, height);
	let img = new Uint8Array(idata.data.buffer);
	//if (imgelem.value)
	//	imgelem.value.src = canvas.toDataURL("image/png")
	//images.push(img);
	return img
}
function videoend() {
	console.log("END")
	if (videoelem.value) videoelem.value.style.display = "none"
	let result = read(images, dimensions.width, dimensions.height, true, threshold.value)
	let url = URL.createObjectURL(new Blob([result], { type: "image/png" }))
	if (imgelem.value && videoelem.value) {
		//videoelem.value.style.display = "none"
		//videoelem.value.src = ""
		imgelem.value.src = url
	}
}

</script>

<template>
	<SWReload />
	<span>Threshold {{ threshold }}: </span>
	<br>
	<input type="number" name="" id="" min="0" max="255" :value="threshold"
		@input="e => threshold = parseInt((e.target as HTMLInputElement).value)">
	<br>
	<span>%Frame {{ frameinterval }}</span>
	<br>
	<input type="number" name="" id="" min="0" max="60" :value="frameinterval"
		@input="e => frameinterval = parseInt((e.target as HTMLInputElement).value)">
	<br>
	<input type="file" name="" id="" @change="fileadd" ref="inputelem">
	<br>
	<button @click="start">Start</button>
	<button @click="startRt">RT</button>
	<br>
	<video controls :src="videourl" ref="videoelem" @canplaythrough="canplay"></video>
	<img src="" alt="" ref="targetimg">
	<HelloWorld msg="Vite + Vue" />
</template>

<style scoped></style>
