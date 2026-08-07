import { createWebHashHistory, createRouter } from 'vue-router'

import HomeView from '../views/index.vue'
import Terminal from '../views/terminal.vue'
import Config from '../views/config.vue'
import Server from '../views/server.vue'


const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    { 
      path: '/', 
      name:'home',
      component: HomeView 
    },{ 
      path: '/terminal', 
      name:'terminal',
      component: Terminal 
    },{ 
      path: '/config/:operation', 
      name:'config',
      component: Config 
    },{ 
      path: '/server', 
      name:'Server',
      component: Server 
    }
  ],
})

export default router