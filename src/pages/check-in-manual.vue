<script setup lang="ts">
import CheckInForm from "@/modules/CheckInForm.vue";
</script>

<template>
  <div class="flex justify-center items-center h-screen relative">
    <!-- Icon in top-left corner -->
    <a
      href="/"
      class="fixed top-5 left-5 z-50 hover:scale-105 transition-transform flex items-center gap-4 group"
    >
      <img
        src="@/assets/images/check-in-manual.png"
        alt="Manual CheckIn"
        width="50"
        height="50"
        class="drop-shadow-xl drop-shadow-emerald-500/50"
      />
      <div class="relative overflow-hidden h-7 flex items-center">
        <span
          class="text-lg font-bold text-emerald-500 drop-shadow-md drop-shadow-emerald-500/50 whitespace-nowrap transition-all duration-300 ease-out group-hover:-translate-y-full group-hover:opacity-0"
          >Manual CheckIn</span
        >
        <span
          class="text-lg font-bold text-emerald-500 drop-shadow-md drop-shadow-emerald-500/50 whitespace-nowrap absolute top-full transition-all duration-300 ease-out group-hover:top-0 group-hover:opacity-100 opacity-0"
          >Back Home</span
        >
      </div>
    </a>

    <CheckInForm />
  </div>
</template>
