<script setup lang="ts">
import "vue-sonner/style.css";
import { Toaster } from "@/components/ui/sonner";
import MatrixBackground from "@/modules/MatrixBackground.vue";
</script>

<template>
  <MatrixBackground />
  <RouterView />
  <Toaster
    rich-colors
    :close-button="true"
    :expand="true"
    position="top-right"
    theme="dark"
    close-button-position="top-right"
  />
</template>
