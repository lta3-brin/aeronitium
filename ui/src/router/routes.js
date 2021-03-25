
const routes = [
  {
    path: '/',
    component: () => import('layouts/default/MainLayout.vue'),
    children: [
      { path: '', name: 'home', component: () => import('pages/home/Home.vue') },
      { path: '/configs', name: 'configs', component: () => import('pages/configs/Configurations.vue') }
    ]
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '*',
    component: () => import('pages/Error404.vue')
  }
]

export default routes
