import {createRouter, createWebHashHistory} from "vue-router";
import Argon2 from "./components/Argon2.vue";
import Exif from "./components/Exif.vue";
import Home from "./components/Home.vue";

const routes = [
    {path: '/', component: Home},
    {path: '/argon2', component: Argon2},
    {path: '/exif', component: Exif},
]

export const router = createRouter({
    history: createWebHashHistory(),
    routes,
})
