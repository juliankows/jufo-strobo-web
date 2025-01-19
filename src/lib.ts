import { ref } from "vue";

let offline = ref(false)

function updateOffline() {
    offline.value = !navigator.onLine
}
window.addEventListener("online", updateOffline)
window.addEventListener("offline", updateOffline)
updateOffline()

export { offline };