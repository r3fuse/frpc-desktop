import { createWebHashHistory, createRouter } from "vue-router";

import IndexView from "../views/index.vue";
import Terminal from "../components/terminal/index.vue";
import Config from "../views/config.vue";
import Server from "../views/server.vue";
import About from "../views/about.vue";
import Status from "../views/status/index.vue";
import Home from "../views/home/index.vue";

const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "index",
            component: IndexView,
            children: [
                {
                    path: "/",
                    name: "home",
                    component: Home,
                },
                {
                    path: "/status",
                    name: "status",
                    component: Status,
                },
                {
                    path: "/about",
                    name: "about",
                    component: About,
                },
                {
                    path: "/server",
                    name: "Server",
                    component: Server,
                },
                {
                    path: "/terminal",
                    name: "terminal",
                    component: Terminal,
                },
            ],
        },

        {
            path: "/config/:operation",
            name: "config",
            component: Config,
        },
    ],
});

export default router;
