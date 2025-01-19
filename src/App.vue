<script setup lang="ts">
import { Ref, ref, useTemplateRef } from 'vue';
import SWReload from './components/SWReload.vue';
import Droptarget from './components/Droptarget.vue';
import MyFooter from './components/MyFooter.vue';


// refs
let videoelem = useTemplateRef("videoelem")

//options
let threshold = ref(40);
let frameinterval = ref(10);
let firstframref = ref(true);
let stage: Ref<"file" | "options" | "read" | "result"> = ref("file");
let isRt = ref(false);
let RtRunning = ref(false);
let resultblob = ref("");

let trimstart = ref("0");
let trimend = ref("100");

let progress = ref(0);

// let log = ref("")


let file = ref<File | null>(null);

// let videoelem = document.createElement("video")

function filechange() {
	console.log(file.value)
	if (file.value) {
		let blob = URL.createObjectURL(file.value);
		if (!videoelem.value) return;
		videoelem.value.src = blob;
		dimensions = { width: 0, height: 0 }
		stage.value = "options"
	} else {
		stage.value = "file"
	}
}



let canvas = document.createElement("canvas")
// let canvas = new OffscreenCanvas(0, 0);
let twod = canvas.getContext("2d", { alpha: false, willReadFrequently: true })

let counter = 0; // no ref

let dimensions = { width: 0, height: 0 }

let camstream: MediaStream | null = null;

async function canplay() {
	if (stage.value == "read" && dimensions.width > 0) return;
	if (!videoelem.value) return
	let height = videoelem.value?.videoHeight ? videoelem.value?.videoHeight : 0;
	let width = videoelem.value?.videoWidth ? videoelem.value?.videoWidth : 0;
	canvas.height = height;
	canvas.width = width;
	dimensions = { width, height }
	// canvas = new OffscreenCanvas(width, height);
	console.log(dimensions)
	if (!isRt.value) {
		videoelem.value.volume = 0
		videoelem.value.playbackRate = 0.1
		videoelem.value.loop = false
		await videoelem.value.play()
		videoelem.value.pause()
	}
}


async function start() {
	stage.value = "read"
	isRt.value = false;
	if (!videoelem.value) return
	// images = []
	comparetarget = null;
	finalimg = null; // reset for next run
	let vid = videoelem.value
	resultblob.value = ""
	let start = (parseFloat(trimstart.value) / 100) * vid.duration;
	let end = (parseFloat(trimend.value) / 100) * vid.duration;
	vid.currentTime = start
	let frametime = 1 / frameinterval.value
	while (vid.currentTime + frametime + 0.05 < end) {
		console.log(`capture ${vid.currentTime}`)
		vid.currentTime += frametime
		progress.value = (vid.currentTime - start) / (end - start);
		await vid.play()
		vid.pause()
		await delay((frametime * 1000))
		let img = await capture()
		if (img) compare(img)
		await delay(5)
	}
	videoend()
}

let finalimg: Uint8Array | null = null;
let comparetarget: null | Uint8Array = null;

function compare(img: Uint8Array) {
	console.log("comparing");
	if (!finalimg) {
		comparetarget = img;
		finalimg = new Uint8Array(img);
		for (let i = 0; i < img.length; i += 4) {
			finalimg[i + 3] = 0;
		}
		console.log("first image skipped")
		return;
	}
	if (!comparetarget) return;
	let thres = threshold.value; // don't read proxy all the time
	for (let i = 0; i < img.length; i += 4) { // rgba
		let diffr = img[i] > comparetarget[i] ? img[i] - comparetarget[i] : comparetarget[i] - img[i];
		let diffg = img[i + 1] > comparetarget[i + 1] ? img[i + 1] - comparetarget[i + 1] : comparetarget[i + 1] - img[i + 1];
		let diffb = img[i + 2] > comparetarget[i + 2] ? img[i + 2] - comparetarget[i + 2] : comparetarget[i + 2] - img[i + 2];
		let max = Math.max(diffr, Math.max(diffg, diffb));
		if (max > thres) {
			if (finalimg[i + 3] > 128) continue; // already masked
			finalimg[i] = img[i];
			finalimg[i + 1] = img[i + 1];
			finalimg[i + 2] = img[i + 2];
			finalimg[i + 3] = 255;
		}
	}
	if (!firstframref.value) comparetarget = img;
}
let rtiv: number = 0;

async function selectRT() {
	if (!videoelem.value) return;
	camstream = await navigator.mediaDevices.getUserMedia({ video: { facingMode: "environment" } })
	if (!camstream) {
		restart();
		return;
	}
	isRt.value = true;
	RtRunning.value = false;
	finalimg = null;
	comparetarget = null;
	videoelem.value.srcObject = camstream
	dimensions = { width: videoelem.value.width, height: videoelem.value.height }
	await videoelem.value.play()
	stage.value = 'read';
}

function startRt() {
	RtRunning.value = true;
	rtiv = setInterval(async () => {
		let img = await capture()
		if (!img) return;
		compare(img)
		console.log("rt")
	}, (1 / frameinterval.value) * 1000)
}

function stopRt() {
	clearInterval(rtiv);
	videoend()
	RtRunning.value = false
	isRt.value = false
	if (videoelem.value) {
		videoelem.value.src = ""
		videoelem.value.srcObject = null
		dimensions = { width: 0, height: 0 }
	}
	camstream?.getTracks().forEach(x => x.stop())
}


function delay(time: number) {
	return new Promise((resolve) => setTimeout(resolve, time))
}

// let images: Uint8Array[] = [];

async function capture() {
	if (!videoelem.value || !twod) return;
	if (counter % frameinterval.value != 0) return;
	let { width, height } = dimensions
	//twod.clearRect(0, 0, width, height)
	twod.drawImage(videoelem.value, 0, 0, width, height)
	let idata = twod.getImageData(0, 0, width, height);
	let img = new Uint8Array(idata.data.buffer);
	return img
}
async function videoend() {
	console.log("END")
	if (!finalimg) return;
	for (let i = 0; i < finalimg.length; i += 4) {
		finalimg[i + 3] = 255;
	}
	let id = new ImageData(new Uint8ClampedArray(finalimg), dimensions.width, dimensions.height);
	twod?.putImageData(id, 0, 0);
	stage.value = "result"
	let url = canvas.toDataURL("image/png");
	// let blob = await canvas.convertToBlob()
	resultblob.value = url//URL.createObjectURL(blob);

}
function download() {
	let link = document.createElement("a");
	link.href = resultblob.value
	let name = "";
	if (file.value && file.value.name) {
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

function trim_move(ev: Event) {
	let target: HTMLInputElement = ev.target as HTMLInputElement;
	if (!videoelem.value) return;
	let part = parseFloat(target.value) / 100
	videoelem.value.currentTime = part * videoelem.value.duration;
}


</script>

<template>

	<!-- <span>{{ stage }} {{ isRt }} {{ RtRunning }}</span> -->
	<!-- <span>{{ log }}</span> -->
	<SWReload />
	<div class="app">
		<div class="spacer"></div>
		<Droptarget @changed="filechange" v-show="stage == 'file' || stage == 'options'" v-model="file" />
		<div class="spacer center">
			<button @click="selectRT" v-if="stage == 'file'" class="center">🎥 Kamera verwenden</button>
		</div>
		<div class="options" v-show="stage == 'options' || (isRt && !RtRunning)">
			<span>Schwellwert</span>
			<br>
			<input type="number" name="" id="" min="0" max="255" :value="threshold"
				@input="e => threshold = parseInt((e.target as HTMLInputElement).value)">
			<br>
			<span>Bildwiderholrate</span>
			<br>
			<input type="number" name="" id="" min="0" max="60" :value="frameinterval"
				@input="e => frameinterval = parseInt((e.target as HTMLInputElement).value)">
			<br>
			<span>Erster Frame als Referenz:</span>
			<input type="checkbox" name="" id="" v-model="firstframref">
		</div>
		<progress v-show="stage == 'read' && !isRt" :value="progress" min="0" max="1"></progress>
		<video src="" ref="videoelem" @canplaythrough="canplay" v-show="stage == 'read' || stage == 'options'"></video>
		<div v-show="stage == 'options'" class="trimmer">
			<span>Start:</span>
			<input type="range" v-model="trimstart" @input="trim_move" min="0" max="100" step="0.1">
			<span>Ende:</span>
			<input type="range" v-model="trimend" @input="trim_move" min="0" max="100" step="0.1">
			<button @click="start" v-show="!isRt" class="center">Start</button>
		</div>
		<div class="rtButton">
			<button v-if="isRt && stage == 'read' && RtRunning" @click="stopRt" class="rtButton">⏹️ Stop</button>
			<button v-if="isRt && stage == 'read' && !RtRunning" @click="startRt" class="rtButton">🔴 Start</button>
		</div>
		<img :src="resultblob" alt="" v-if="stage == 'result'" class="result">
		<button class="center" @click="download" v-if="stage == 'result'">💾 Download</button>
		<div class="spacer"></div>
		<button class="center" @click="restart" v-if="stage == 'result'">Weiters Bild erstellen</button>
		<div class="spacer"></div>
	</div>
	<MyFooter></MyFooter>
</template>

<style scoped>
.app {
	overflow-x: auto;
	height: calc(100vh - 3.5rem);
}

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
	display: block;
	margin: 0.5rem auto;
	margin-top: 1rem;
	max-width: calc(100vw - 1rem);
	max-height: calc(100vh - 10rem);
}

.trimmer input {
	display: block;
	width: calc(100vw - 2rem);
	margin: 0 auto;
	margin-bottom: 1rem;
}

.result {
	display: block;
	margin: .5rem auto;
	max-width: calc(100vw - 1rem);
	max-height: calc(100vh - 6rem);
}

progress {
	display: block;
	margin: 0.5rem auto;
	width: calc(100% - 4rem);
}

div.rtButton {

	position: absolute;
	top: 0;
	right: 2rem;
	height: 100vh;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
}

button.rtButton {
	padding: 0;
	width: 5rem;
	height: 3rem;
}
</style>
