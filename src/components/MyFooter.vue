<script lang="ts" setup>
import { useRegisterSW } from 'virtual:pwa-register/vue';
import {offline} from '../lib'

let {updateServiceWorker, needRefresh} = useRegisterSW()

function refresh() {
    if(needRefresh) {
        updateServiceWorker()
    } else {
        window.location.reload()
    }
}
let version = GIT_VERSION
</script>

<template>
    <div class="footer">
        <span>{{  offline ? "💾" : "🌐" }}</span>
        <span class="center">Stroboskopaufnahmen <span class="version">{{ version }}</span></span>
        <a @click="refresh" :href="needRefresh ? 'javascript:' : ''">{{needRefresh ? "Update" :"Neu Laden"}}</a>
    </div>
</template>

<style scoped>
.footer {
    bottom: 0rem;
    background-color: #ccc;
    height: 2rem;
		position: fixed;
    padding: .75rem 1rem;
    width: calc(100vw - 2rem);
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    align-items: center;
}
.footer > * {
    width: 33vw;
}
.version {
    color: #777;
}
span.center {
    text-align: center;
}
a {
    display: inline-block;
    color: #777;
    /* margin-left: 1rem; */
    text-align: right;
    text-decoration: underline;
}

a:visited {
    color: #777;
}
.space {
    margin-left: 1rem;
}
</style>
