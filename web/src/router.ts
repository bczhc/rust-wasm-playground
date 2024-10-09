import {createRouter, createWebHashHistory} from "vue-router";
import Argon2 from "./components/Argon2.vue";
import Exif from "./components/Exif.vue";
import Home from "./components/Home.vue";
import Ipv6Xor from "./components/Ipv6Xor.vue";
import RustParser from "./components/RustParser.vue";

export const routes = [
    {path: '/', component: Home},
    {path: '/argon2', component: Argon2},
    {path: '/exif', component: Exif},
    {path: '/ipv6-xor', component: Ipv6Xor},
    {path: '/rust-parser', component: RustParser}
]

export const router = createRouter({
    history: createWebHashHistory(),
    routes,
})
