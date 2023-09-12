import { createRouter, createWebHashHistory } from 'vue-router';
import App from '../App.vue';
import HomePage from '../pages/Home/index.vue';
import WeReadV1 from '../pages/WeReadV1/index.vue';
import WeReadV2 from '../pages/WeReadV2/index.vue';
import Snipaste from '../pages/Snipaste/index.vue';

export const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: '/',
            component: App,
            children: [
                {
                    path: '/',
                    component: HomePage
                },
                {
                    path: '/we-read-v1',
                    component: WeReadV1
                },
                {
                    path: '/we-read-v2',
                    component: WeReadV2
                },
                {
                    path: '/snipaste',
                    component: Snipaste
                }
            ]
        }
    ]
})
