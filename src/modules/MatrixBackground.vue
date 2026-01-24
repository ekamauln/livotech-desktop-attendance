<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

const canvas = ref<HTMLCanvasElement | null>(null);
let animationId: number | null = null;

onMounted(() => {
  if (!canvas.value) return;

  const ctx = canvas.value.getContext("2d");
  if (!ctx) return;

  // Set canvas size to window size
  canvas.value.width = window.innerWidth;
  canvas.value.height = window.innerHeight;

  // Matrix characters
  const chars = "01";
  const fontSize = 16;
  const columns = Math.floor(canvas.value.width / fontSize);

  // Array to track y position of each column
  const drops: number[] = Array(columns).fill(1);

  // Animation function
  const draw = () => {
    if (!canvas.value || !ctx) return;

    // Semi-transparent black to create fade effect
    ctx.fillStyle = "rgba(15, 23, 42, 0.05)"; // stone-900 with low opacity
    ctx.fillRect(0, 0, canvas.value.width, canvas.value.height);

    // Set text style
    ctx.fillStyle = "#0f766e";
    ctx.font = `${fontSize}px monospace`;

    // Loop through drops
    for (let i = 0; i < drops.length; i++) {
      // Random character
      const text = chars[Math.floor(Math.random() * chars.length)] || "0";
      const x = i * fontSize;
      const y = drops[i]! * fontSize;

      ctx.fillText(text, x, y);

      // Reset drop to top randomly
      if (y > canvas.value.height && Math.random() > 0.975) {
        drops[i] = 0;
      }

      // Increment y coordinate
      drops[i]!++;
    }

    animationId = requestAnimationFrame(draw);
  };

  // Start animation
  draw();

  // Handle window resize
  const handleResize = () => {
    if (!canvas.value) return;
    canvas.value.width = window.innerWidth;
    canvas.value.height = window.innerHeight;
  };

  window.addEventListener("resize", handleResize);

  // Cleanup
  onUnmounted(() => {
    if (animationId) {
      cancelAnimationFrame(animationId);
    }
    window.removeEventListener("resize", handleResize);
  });
});
</script>

<template>
  <canvas
    ref="canvas"
    class="fixed top-0 left-0 w-full h-full pointer-events-none -z-10"
  />
</template>
