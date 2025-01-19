<script lang="ts" setup>
// import { useRegisterSW } from 'virtual:pwa-register/vue';
import { offline } from '../lib'

// let sw: ServiceWorkerRegistration | null = null;
// let { updateServiceWorker, needRefresh } = useRegisterSW({
//     onRegisteredSW: (_, s) => {
//         if (!s) return;
//         sw = s;
//     }
// })


// async function refresh() {
//     console.log("click")
//     if (needRefresh.value) {
//         console.log("update")
//         updateServiceWorker()
//     } else {
//         console.log("refresh")
//         if (!offline.value) await clear()
//         window.location.reload()
//     }
// }
// async function clear() {
//     console.log("clearing")
//     await sw?.unregister()
//     let keys = await caches.keys();
//     for (let key of keys) {
//         await caches.delete(key)
//     }
//     updateServiceWorker()
// }
let version = GIT_VERSION
</script>

<template>
    <div class="footer">
        <span class="online">{{ offline ? "💾" : "🌐" }}</span>
        <span class="version center">{{ version }}</span>
        <!-- <a class="right reload" @click="refresh" href="javascript:">{{ needRefresh ? "Update" : -->
            <!-- "Neu Laden" }}</a> -->
    </div>
</template>

<style scoped>
.online {
    text-align: left;
    margin-right: 0.5rem;
}

.footer {
    bottom: 0rem;
    background-color: #ddd;
    height: 2rem;
    position: fixed;
    padding: .75rem 1rem;
    width: calc(100vw - 2rem);
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
}

.version {
    color: #777;
}

span.center {
    text-align: center;
}

.reload {
    color: #777;
    text-decoration: underline;
}

.reload:visited {
    color: #777;
}

a.right {
    text-align: right;
}

.space {
    margin-left: 1rem;
}
</style>
