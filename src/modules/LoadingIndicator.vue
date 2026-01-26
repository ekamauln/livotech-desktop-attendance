<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";

const isLoading = ref(false);
const progress = ref(0);
const timer = ref<number | null>(null);

const router = useRouter();

const startLoading = () => {
  isLoading.value = true;
  progress.value = 0;

  if (timer.value) {
    clearInterval(timer.value);
  }

  // Simulate progress
  timer.value = window.setInterval(() => {
    if (progress.value < 90) {
      progress.value += Math.random() * 10;
    }
  }, 200);
};

const finishLoading = () => {
  if (timer.value) {
    clearInterval(timer.value);
    timer.value = null;
  }

  progress.value = 100;

  setTimeout(() => {
    isLoading.value = false;
    progress.value = 0;
  }, 400);
};

const failLoading = () => {
  if (timer.value) {
    clearInterval(timer.value);
    timer.value = null;
  }

  progress.value = 100;

  setTimeout(() => {
    isLoading.value = false;
    progress.value = 0;
  }, 400);
};

// Router navigation guards
router.beforeEach((to, from, next) => {
  if (to.path !== from.path) {
    startLoading();
  }
  next();
});

router.afterEach(() => {
  finishLoading();
});

router.onError(() => {
  failLoading();
});
</script>

<template>
  <div
    v-if="isLoading"
    class="loading-indicator"
    :style="{ width: `${progress}%` }"
  />
</template>

<style scoped>
.loading-indicator {
  position: fixed;
  top: 0;
  left: 0;
  height: 3px;
  background: linear-gradient(90deg, #10b981 0%, #0ea5e9 50%, #ef4444 100%);
  transition:
    width 0.2s ease,
    opacity 0.4s ease;
  z-index: 9999;
  box-shadow: 0 0 10px rgba(16, 185, 129, 0.5);
}
</style>
