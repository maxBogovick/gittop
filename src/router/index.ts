import { createRouter, createWebHistory } from 'vue-router';
import TopRepositoriesView from '../views/TopRepositoriesView.vue';
import NewRepositoriesView from '../views/NewRepositoriesView.vue';
import RedditTopView from '../views/RedditTopView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/top'
    },
    {
      path: '/top',
      name: 'top',
      component: TopRepositoriesView
    },
    {
      path: '/new',
      name: 'new',
      component: NewRepositoriesView
    },
    {
      path: '/reddit/top',
      name: 'reddit-top',
      component: RedditTopView
    },
    {
      path: '/reddit/new',
      name: 'reddit-new',
    component: () => import('../views/RedditNewView.vue')
  },
  {
    path: '/devto/top',
    name: 'devto-top',
    component: () => import('../views/DevtoTopView.vue')
  },
  {
    path: '/devto/new',
    name: 'devto-new',
    component: () => import('../views/DevtoNewView.vue')
  },
  {
    path: '/etsy/top',
    name: 'etsy-top',
    component: () => import('../views/EtsyTopView.vue')
  },
  {
    path: '/etsy/new',
    name: 'etsy-new',
    component: () => import('../views/EtsyNewView.vue')
  },
  {
    path: '/stackoverflow',
    name: 'stackoverflow',
    component: () => import('../views/StackOverflowView.vue')
  },
  {
    path: '/hackernews',
    name: 'hackernews',
    component: () => import('../views/HackerNewsView.vue')
  },
  {
    path: '/medium',
    name: 'medium',
    component: () => import('../views/MediumView.vue')
  },
  {
    path: '/hashnode',
    name: 'hashnode',
    component: () => import('../views/HashnodeView.vue')
  },
  {
    path: '/producthunt',
    name: 'producthunt',
    component: () => import('../views/ProductHuntView.vue')
  },
  {
    path: '/lobsters',
    name: 'lobsters',
    component: () => import('../views/LobstersView.vue')
  },
  {
    path: '/crates',
    name: 'crates',
    component: () => import('../views/CratesView.vue')
  },
  {
    path: '/indiehackers',
    name: 'indiehackers',
    component: () => import('../views/IndieHackersView.vue')
  }
]
});

export default router;
