import { createWebHashHistory, createRouter } from "vue-router";

import HomeView from "../views/index.vue";
import Terminal from "../views/terminal.vue";
import Config from "../views/config.vue";
import Server from "../views/server.vue";
import About from "../views/about.vue";
import Status from "../views/status/index.vue";

const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: HomeView,
            children: [
                {
                    path: "/",
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
