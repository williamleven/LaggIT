import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Home',
      component: Home,
    },
    {
      path: '/events',
      name: 'Events',
      component: () => import(/* webpackChunkName: "events" */ './views/Events.vue'),
    },
    {
      path: '/event/:id',
      name: 'Event',
      component: () => import(/* webpackChunkName: "events" */ './views/Event.vue'),
    },
    {
      path: '/member',
      name: 'Member',
      component: () => import(/* webpackChunkName: "member" */ './views/Member.vue'),
    },
    {
      path: '/about',
      name: 'About',
      component: () => import(/* webpackChunkName: "about" */ './views/About.vue'),
    },
  ],
})
