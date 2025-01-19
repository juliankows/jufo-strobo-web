<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import { init, read } from '../wasm/pkg'
import { onMounted, ref, useTemplateRef } from 'vue';
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
let twod = canvas.getContext("2d")
let dimensions = { width: 0, height: 0 }
function canplay() {
	if (!videoelem.value) return
	videoelem.value.requestVideoFrameCallback(vfc)
	let height = videoelem.value?.videoHeight ? videoelem.value?.videoHeight : 0;
	let width = videoelem.value?.videoWidth ? videoelem.value?.videoWidth : 0;
	canvas.height = height;
	canvas.width = width;
	dimensions = { width, height }
	videoelem.value.volume = 0
	videoelem.value.play()
}
let images: Uint8Array[] = [];
function vfc() {
	counter++;
	console.log(counter);
	if (!videoelem.value || !twod) return;
	videoelem.value?.requestVideoFrameCallback(vfc)
	if (counter % frameinterval.value != 0) return;
	videoelem.value.pause();
	console.log("VFC")
	let { width, height } = dimensions
	twod.drawImage(videoelem.value, 0, 0, width, height)
	let idata = twod.getImageData(0, 0, width, height);
	let img = new Uint8Array(idata.data.buffer);
	images.push(img);
	videoelem.value.play()
}
function videoend() {
	console.log("END")
	//videoelem.value.style.display = "none"
	let result = read(images, dimensions.width, dimensions.height, true, threshold.value)
	let url = URL.createObjectURL(new Blob([result], { type: "image/png" }))
	if (imgelem.value && videoelem.value) {
		videoelem.value.style.display = "none"
		videoelem.value.src = ""
		imgelem.value.src = url
	}
}

</script>

<template>
	<span>Threshold {{ threshold }}: </span>
	<br>
	<input type="range" name="" id="" min="0" max="255" :value="threshold"
		@input="e => threshold = parseInt((e.target as HTMLInputElement).value)">
	<br>
	<span>%Frame {{ frameinterval }}</span>
	<br>
	<input type="range" name="" id="" min="0" max="60" :value="frameinterval"
		@input="e => frameinterval = parseInt((e.target as HTMLInputElement).value)">
	<br>
	<input type="file" name="" id="" @change="fileadd" ref="inputelem">
	<br>
	<video :src="videourl" ref="videoelem" @canplaythrough="canplay" @ended="videoend"></video>
	<img src="" alt="" ref="targetimg">
	<HelloWorld msg="Vite + Vue" />
</template>

<style scoped></style>
