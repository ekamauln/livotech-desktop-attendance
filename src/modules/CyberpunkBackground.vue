<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

const canvas = ref<HTMLCanvasElement | null>(null);
let animationId: number | null = null;

interface Doodle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  color: string;
  alpha: number;
  fadeSpeed: number;
  rotation: number;
  rotationSpeed: number;
  type: "circle" | "square" | "triangle" | "line" | "plus" | "x" | "hexagon";
}

onMounted(() => {
  if (!canvas.value) return;

  const ctx = canvas.value.getContext("2d");
  if (!ctx) return;

  // Set canvas size to window size
  canvas.value.width = window.innerWidth;
  canvas.value.height = window.innerHeight;

  // Cyberpunk color palette
  const colors = [
    "#ff006e", // Neon pink
    "#ff0080", // Hot pink
    "#00d9ff", // Cyan
    "#00ffff", // Aqua
    "#b300ff", // Purple
    "#8b00ff", // Violet
    "#39ff14", // Neon green
    "#ffff00", // Yellow
    "#ff3864", // Red pink
  ];

  const doodleTypes: Doodle["type"][] = [
    "circle",
    "square",
    "triangle",
    "line",
    "plus",
    "x",
    "hexagon",
  ];

  // Array to store doodles
  const doodles: Doodle[] = [];
  const maxDoodles = 50;

  // Create initial doodles
  const createDoodle = (): Doodle => {
    return {
      x: Math.random() * canvas.value!.width,
      y: Math.random() * canvas.value!.height,
      vx: (Math.random() - 0.5) * 2,
      vy: (Math.random() - 0.5) * 2,
      size: Math.random() * 30 + 10,
      color: colors[Math.floor(Math.random() * colors.length)]!,
      alpha: Math.random() * 0.5 + 0.3,
      fadeSpeed: Math.random() * 0.01 + 0.002,
      rotation: Math.random() * Math.PI * 2,
      rotationSpeed: (Math.random() - 0.5) * 0.05,
      type: doodleTypes[Math.floor(Math.random() * doodleTypes.length)]!,
    };
  };

  // Draw different doodle types
  const drawDoodle = (doodle: Doodle) => {
    if (!ctx) return;

    ctx.save();
    ctx.translate(doodle.x, doodle.y);
    ctx.rotate(doodle.rotation);
    ctx.globalAlpha = doodle.alpha;
    ctx.strokeStyle = doodle.color;
    ctx.fillStyle = doodle.color;
    ctx.lineWidth = 2;

    const size = doodle.size;

    switch (doodle.type) {
      case "circle":
        ctx.beginPath();
        ctx.arc(0, 0, size / 2, 0, Math.PI * 2);
        ctx.stroke();
        break;

      case "square":
        ctx.beginPath();
        ctx.rect(-size / 2, -size / 2, size, size);
        ctx.stroke();
        break;

      case "triangle":
        ctx.beginPath();
        ctx.moveTo(0, -size / 2);
        ctx.lineTo(size / 2, size / 2);
        ctx.lineTo(-size / 2, size / 2);
        ctx.closePath();
        ctx.stroke();
        break;

      case "line":
        ctx.beginPath();
        ctx.moveTo(-size / 2, 0);
        ctx.lineTo(size / 2, 0);
        ctx.stroke();
        break;

      case "plus":
        ctx.beginPath();
        ctx.moveTo(-size / 2, 0);
        ctx.lineTo(size / 2, 0);
        ctx.moveTo(0, -size / 2);
        ctx.lineTo(0, size / 2);
        ctx.stroke();
        break;

      case "x":
        ctx.beginPath();
        ctx.moveTo(-size / 2, -size / 2);
        ctx.lineTo(size / 2, size / 2);
        ctx.moveTo(size / 2, -size / 2);
        ctx.lineTo(-size / 2, size / 2);
        ctx.stroke();
        break;

      case "hexagon":
        ctx.beginPath();
        for (let i = 0; i < 6; i++) {
          const angle = (Math.PI / 3) * i;
          const x = (size / 2) * Math.cos(angle);
          const y = (size / 2) * Math.sin(angle);
          if (i === 0) {
            ctx.moveTo(x, y);
          } else {
            ctx.lineTo(x, y);
          }
        }
        ctx.closePath();
        ctx.stroke();
        break;
    }

    ctx.restore();
  };

  // Animation function
  const draw = () => {
    if (!canvas.value || !ctx) return;

    // Clear canvas with dark background
    ctx.fillStyle = "rgba(10, 14, 39, 0.1)"; // Dark cyberpunk background with fade
    ctx.fillRect(0, 0, canvas.value.width, canvas.value.height);

    // Add new doodles randomly
    if (doodles.length < maxDoodles && Math.random() < 0.1) {
      doodles.push(createDoodle());
    }

    // Update and draw doodles
    for (let i = doodles.length - 1; i >= 0; i--) {
      const doodle = doodles[i]!;

      // Update position
      doodle.x += doodle.vx;
      doodle.y += doodle.vy;

      // Update rotation
      doodle.rotation += doodle.rotationSpeed;

      // Fade out
      doodle.alpha -= doodle.fadeSpeed;

      // Draw doodle
      drawDoodle(doodle);

      // Remove faded doodles
      if (doodle.alpha <= 0) {
        doodles.splice(i, 1);
      }

      // Wrap around screen edges
      if (doodle.x < -50) doodle.x = canvas.value.width + 50;
      if (doodle.x > canvas.value.width + 50) doodle.x = -50;
      if (doodle.y < -50) doodle.y = canvas.value.height + 50;
      if (doodle.y > canvas.value.height + 50) doodle.y = -50;
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
