<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { Card, CardContent } from "@/components/ui/card";

interface AnimatingIcon {
  src: string;
  alt: string;
  width: number;
  height: number;
}

const router = useRouter();
const clickedIcon = ref<number | null>(null);
const animatingIcon = ref<AnimatingIcon | null>(null);
const animationStyle = ref<Record<string, string>>({});

const handleClick = (event: MouseEvent, index: number, href: string) => {
  const target = event.currentTarget as HTMLElement;
  if (!target) return;

  const rect = target.getBoundingClientRect();
  const imgElement = target.querySelector("img");
  if (!imgElement) return;

  const imgData = {
    src: imgElement.src,
    alt: imgElement.alt,
    width: 200,
    height: 200,
  };

  // Set initial position (where the icon is)
  animationStyle.value = {
    left: `${rect.left}px`,
    top: `${rect.top}px`,
    width: "200px",
    height: "200px",
  };

  clickedIcon.value = index;
  animatingIcon.value = imgData;

  // Trigger animation after a small delay
  setTimeout(() => {
    animationStyle.value = {
      left: "20px",
      top: "20px",
      width: "50px",
      height: "50px",
    };
  }, 50);

  // Navigate after animation completes
  setTimeout(() => {
    if (href !== "#") {
      router.push(href);
    }
  }, 800);
};
</script>

<template>
  <div
    class="flex flex-col items-center justify-center min-h-screen text-emerald-500 relative overflow-hidden"
  >
    <Card
      class="h-80 p-10 bg-stone-800/20 backdrop-blur-xs border-2 border-emerald-500 flex items-center drop-shadow-xl drop-shadow-emerald-500/50 transition-opacity duration-300"
      :class="{ 'opacity-0': clickedIcon !== null }"
    >
      <CardContent class="p-0">
        <div class="flex gap-10">
          <a
            href="/check-in-face"
            class="relative flex flex-col items-center group overflow-hidden transition-opacity duration-500"
            :class="{
              'opacity-100': clickedIcon === 0 || clickedIcon === null,
            }"
            @click.prevent="handleClick($event, 0, '/check-in-face')"
          >
            <div
              class="absolute -top-8 opacity-0 transform -translate-y-2 transition-all duration-300 ease-out group-hover:top-0 group-hover:opacity-100 group-hover:translate-y-0 mb-2"
            >
              <span
                class="text-lg font-bold text-emerald-500 drop-shadow-md drop-shadow-emerald-500/50"
                >Face CheckIn</span
              >
            </div>
            <img
              src="@/assets/images/check-in-face.png"
              alt="Face CheckIn"
              width="200"
              height="200"
              class="hover:drop-shadow-xl hover:drop-shadow-emerald-500/50 transition-all duration-100 group-hover:mt-10"
            />
          </a>
          <a
            href="/check-in-manual"
            class="relative flex flex-col items-center group overflow-hidden transition-opacity duration-500"
            :class="{
              'opacity-100': clickedIcon === 1 || clickedIcon === null,
            }"
            @click.prevent="handleClick($event, 1, '/check-in-manual')"
          >
            <div
              class="absolute -top-8 opacity-0 transform -translate-y-2 transition-all duration-300 ease-out group-hover:top-0 group-hover:opacity-100 group-hover:translate-y-0 mb-2"
            >
              <span
                class="text-lg font-bold text-emerald-500 drop-shadow-md drop-shadow-emerald-500/50"
                >Manual CheckIn</span
              >
            </div>
            <img
              src="@/assets/images/check-in-manual.png"
              alt="Manual CheckIn"
              width="200"
              height="200"
              class="hover:drop-shadow-xl hover:drop-shadow-emerald-500/50 transition-all duration-100 group-hover:mt-10"
            />
          </a>
          <a
            href="/check-out-face"
            class="relative flex flex-col items-center group overflow-hidden transition-opacity duration-500"
            :class="{
              'opacity-100': clickedIcon === 2 || clickedIcon === null,
            }"
            @click.prevent="handleClick($event, 2, '/check-out-face')"
          >
            <div
              class="absolute -top-8 opacity-0 transform -translate-y-2 transition-all duration-300 ease-out group-hover:top-0 group-hover:opacity-100 group-hover:translate-y-0 mb-2"
            >
              <span
                class="text-lg font-bold text-red-500 drop-shadow-md drop-shadow-red-500/50"
                >Face CheckOut</span
              >
            </div>
            <img
              src="@/assets/images/check-out-face.png"
              alt="Face CheckOut"
              width="200"
              height="200"
              class="hover:drop-shadow-xl hover:drop-shadow-red-500/50 transition-all duration-100 group-hover:mt-10"
            />
          </a>
          <a
            href="/check-out-manual"
            class="relative flex flex-col items-center group overflow-hidden transition-opacity duration-500"
            :class="{
              'opacity-100': clickedIcon === 3 || clickedIcon === null,
            }"
            @click.prevent="handleClick($event, 3, '/check-out-manual')"
          >
            <div
              class="absolute -top-8 opacity-0 transform -translate-y-2 transition-all duration-300 ease-out group-hover:top-0 group-hover:opacity-100 group-hover:translate-y-0 mb-2"
            >
              <span
                class="text-lg font-bold text-red-500 drop-shadow-md drop-shadow-red-500/50"
                >Manual CheckOut</span
              >
            </div>
            <img
              src="@/assets/images/check-out-manual.png"
              alt="Manual CheckOut"
              width="200"
              height="200"
              class="hover:drop-shadow-xl hover:drop-shadow-red-500/50 transition-all duration-100 group-hover:mt-10"
            />
          </a>
        </div>
      </CardContent>
    </Card>

    <!-- Animated clone -->
    <div
      v-if="animatingIcon !== null"
      class="fixed transition-all duration-700 ease-in-out"
      :style="animationStyle"
    >
      <img
        :src="animatingIcon.src"
        :alt="animatingIcon.alt"
        :width="animatingIcon.width"
        :height="animatingIcon.height"
        class="drop-shadow-xl"
      />
    </div>
  </div>
</template>
