import {createRouter, createWebHashHistory} from "vue-router";
import Argon2 from "./components/Argon2.vue";
import Exif from "./components/Exif.vue";
import Home from "./components/Home.vue";
import Ipv6Xor from "./components/Ipv6Xor.vue";
import RustParser from "./components/RustParser.vue";
import LegCpu from "./components/LegCpu.vue";
import Kaprekar from "./components/Kaprekar.vue";
import Bitcoin from "./components/Bitcoin.vue";

export const routes = [
    {path: '/', component: Home, name: 'Home'},
    {path: '/argon2', component: Argon2, name: 'Argon2'},
    {path: '/exif', component: Exif, name: 'Exif'},
    {path: '/ipv6-xor', component: Ipv6Xor, name: 'Ipv6Xor'},
    {path: '/rust-parser', component: RustParser, name: 'RustParser'},
    {path: '/leg-cpu', component: LegCpu, name: 'LegCpu'},
    {path: '/kaprekar', component: Kaprekar, name: 'Kaprekar'},
    {path: '/bitcoin', component: Bitcoin, name: 'Bitcoin'},
]

export const router = createRouter({
    history: createWebHashHistory(),
    routes,
})
