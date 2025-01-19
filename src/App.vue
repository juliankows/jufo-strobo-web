<script setup lang="ts">
import { init, read } from '../wasm/pkg'
import { onMounted, Ref, ref, useTemplateRef } from 'vue';
import SWReload from './components/SWReload.vue';
import Droptarget from './components/Droptarget.vue';

onMounted(() => {
	init()
})

// refs
let videoelem = useTemplateRef("videoelem")



//options
let threshold = ref(40);
let frameinterval = ref(10);
let stage: Ref<"file" | "options" | "read" | "generate" | "result"> = ref("file");
let resultblob = ref("");

let file = ref<File | null>(null);

// let videoelem = document.createElement("video")

function filechange() {
	console.log(file.value)
	if(file.value) {
		let blob = URL.createObjectURL(file.value);
		if (!videoelem.value) return;
		videoelem.value.src = blob;
		stage.value = "options"
	} else {
		stage.value = "file"
	}
}



let canvas = document.createElement("canvas")
let twod = canvas.getContext("2d", { alpha: false, willReadFrequently: true })

let counter = 0; // no ref

let dimensions = { width: 0, height: 0 }

async function canplay() {
	if (!videoelem.value) return
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
	stage.value = "read"
	if (!videoelem.value) return
	images = []
	let vid = videoelem.value
	resultblob.value = ""
	vid.currentTime = 0
	let frametime = 1 / frameinterval.value
	while (vid.currentTime + frametime + 0.05 < vid.duration) {
		console.log(`capture ${vid.currentTime}`)
		vid.currentTime += frametime
		await vid.play()
		vid.pause()
		await delay((frametime * 1000))
		let img = await capture()
		if (img) images.push(img)
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
	if (!first) return;
	rtcurrent = first;
	rtiv = setInterval(async () => {
		let img = await capture()
		if (!img) return;


	}, (1 / frameinterval.value) * 1000)
}
console.log({ rtiv, rtcurrent })
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
	//twod.clearRect(0, 0, width, height)
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
	stage.value = "generate"
	let result = read(images, dimensions.width, dimensions.height, true, threshold.value)
	stage.value = "result"
	let url = URL.createObjectURL(new Blob([result], { type: "image/png" }))
	resultblob.value = url
}
function download() {
	let link = document.createElement("a");
	link.href = resultblob.value
	let name = "";
	if(file.value && file.value.name) {
		let split = file.value.name.split('.')
		split.pop()
		split.push("png")
		name = split.join(".")
	} else {
		name = "Stroboskopbild.png"
	}
	link.download = name
	link.click()
}
function restart() {
	file.value = null
	stage.value = 'file'
}

</script>

<template>
	<SWReload />
	<div class="spacer"></div>
	<Droptarget @changed="filechange" v-show="stage == 'file' || stage == 'options'" v-model="file"/>
	<div class="spacer center">
		<button @click="startRt" v-if="stage == 'file'" class="center">🎥 Webcam verwenden</button>
	</div>
	<div class="options" v-show="stage == 'options'">
		<span>Schwellwert</span>
		<br>
		<input type="number" name="" id="" min="0" max="255" :value="threshold"
			@input="e => threshold = parseInt((e.target as HTMLInputElement).value)">
		<br>
		<span>Bildwiderholrate</span>
		<br>
		<input type="number" name="" id="" min="0" max="60" :value="frameinterval"
			@input="e => frameinterval = parseInt((e.target as HTMLInputElement).value)">
		<div class="spacer"></div>
		<button @click="start">Start</button>
		<br>
	</div>
	<video src="" ref="videoelem" @canplaythrough="canplay" v-show="stage == 'read'"></video>
	<span class="center" v-if="stage == 'generate'">Stroboskopbild wird erstellet...</span>
	<img :src="resultblob" alt="" v-if="stage == 'result'" class="result">
	<button class="center" @click="download" v-if="stage == 'result'">💾 Download</button>
	<div class="spacer"></div>
	<button class="center" @click="restart" v-if="stage == 'result'">Weiters Bild erstellen</button>
</template>

<style scoped>
.options {
	margin: auto;
	width: fit-content;
	margin-top: 1rem;
	text-align: center;
}

.spacer {
	margin-top: .5rem;
}

.center {
	text-align: center;
	display: block;
	width: fit-content;
	margin-left: auto;
	margin-right: auto;
}

video {
	margin: 0 .5rem;
	max-width: calc(100vw - 1rem);
}

.result {
	margin: 0 .5rem;
	max-width: calc(100vw - 1rem);
}
</style>
