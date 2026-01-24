import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "index",
      component: () => import("@/pages/index.vue"),
    },
    {
      path: "/check-in-face",
      name: "check-in-face",
      component: () => import("@/pages/check-in-face.vue"),
    },
    {
      path: "/check-out-face",
      name: "check-out-face",
      component: () => import("@/pages/check-out-face.vue"),
    },
  ],
});

export default router;
