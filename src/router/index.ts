import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/todos',
    },
    {
      path: '/todos',
      name: 'todos',
      component: () => import('../views/TodosPage.vue'),
    },
    {
      path: '/review',
      name: 'review',
      component: () => import('../views/ReviewPage.vue'),
    },
    {
      path: '/weekly',
      name: 'weekly',
      component: () => import('../views/WeeklyPage.vue'),
    },
    {
      path: '/notes',
      name: 'notes',
      component: () => import('../views/NotesPage.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/SettingsPage.vue'),
    },
  ],
})

export default router
