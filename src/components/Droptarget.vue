<script lang="ts" setup>
import { ref, useTemplateRef, watch } from 'vue';

const dragged = ref(false);
const fileElem = useTemplateRef("file")

const model = defineModel<File | null>()

const haveFile = ref(false);

const errorText = ref("");

const emit = defineEmits({ changed() { return true; } })

function addFileInput() {
	console.log(fileElem.value)
	if (!fileElem.value || !fileElem.value.files || !fileElem.value.files[0]) return
	let files = fileElem.value.files;
	if (files.length != 1) {
		error("Nur eine Datei gleichzeitig möglich");
		return;
	}

	addFile(files[0])
}
function addFile(file: File) {
	console.log(file)
	if (!file.type.startsWith("video")) {
		error("keine Videodatei")
		return;
	}
	haveFile.value = true;
	model.value = file;
	emit('changed')
}

function clear() {
	haveFile.value = false;
	model.value = null;
	if (fileElem.value) fileElem.value.value = '';
	emit("changed")
}

watch(model, (newV, _) => {
	haveFile.value = !!newV;
})

function dropped(ev: DragEvent) {
	ev.preventDefault()
	dragged.value = false;
	if (!ev.dataTransfer) return;
	let files = ev.dataTransfer.items ? [...ev.dataTransfer.items].filter(x => x.kind == "file").map(x => x.getAsFile()) : [...ev.dataTransfer.files].map(x => x);
	if (files.length != 1) {
		error("Nur eine Datei gleichzeitig möglich");
		return;
	}
	if (files[0]) addFile(files[0])
}

function error(msg: string) {
	errorText.value = msg
	setTimeout(() => {
		errorText.value = ""
		dragged.value = false;
	}, 5000)
}

</script>
<template>
	<div class="target" @dragenter="dragged = true" @dragleave="dragged = false" @dragover.stop.prevent
		@drop.stop.prevent="dropped" :class="{ dragged, haveFile }">
		<input id="" ref="file" type="file" name="" style="display: none" @change="addFileInput" accept="video/*">
		<div class="center" v-if="!haveFile" @dragenter="dragged = true" @dragleave="dragged = false"
			@drop.prevent="dropped">
			<span>Datei hierher ziehen</span>
			<span>oder</span>
			<button class="select" @click="fileElem?.click">📼 Video Auswählen</button>
		</div>
		<div class="center" v-if="haveFile">
			<button @click="clear">Datei Entfernen</button>
		</div>
		<div class="center" v-if="errorText">
			<span class="error">{{ errorText }}</span>
		</div>
	</div>
</template>

<style scoped>
.target {
	width: 50vw;
	min-width: 5rem;
	margin: auto;

	display: block;
}

.target:not(.haveFile) {
	height: 10rem;
	max-height: 50vh;
	border: 1px dashed #777;
	border-radius: 5px;
}


.target.dragged {
	background-color: #888;
}

.center {
	text-align: center;
	width: fit-content;
	margin: auto;
}

.center span {
	margin-bottom: 1rem;
	display: block;
}

.center>*:first-child {
	margin-top: 1rem;
}

.error {
	color: red;
}
</style>
