<script setup lang="ts">
import * as faceapi from "face-api.js";
import { onMounted, ref } from "vue";
import { toast } from "vue-sonner";
import { useTTS } from "@/lib/useTTS";
import { fetch } from "@tauri-apps/plugin-http";
import { Card, CardContent } from "@/components/ui/card";

const video = ref<HTMLVideoElement | null>(null);
const canvas = ref<HTMLCanvasElement | null>(null);
const isVerifying = ref(false);
const lastVerificationTime = ref<number>(0);
const VERIFICATION_COOLDOWN = 2000; // 2 seconds cooldown between auto-verifications
const faceDetectedTime = ref<number>(0);
const DETECTION_DELAY = 1000; // 1s delay after stable face detection before verification
const statusText = ref<string>("Position your face");
const countdown = ref<number>(0);

// Initialize TTS (Text-to-Speech) - though not used in this component yet
const { speak, unlockAudio } = useTTS();

async function loadModels() {
  const MODEL_URL = "/models";

  await faceapi.nets.tinyFaceDetector.loadFromUri(MODEL_URL);
  // Optional later:
  // await faceapi.nets.faceLandmark68Net.loadFromUri(MODEL_URL)
}

async function startCamera() {
  const stream = await navigator.mediaDevices.getUserMedia({
    video: true,
  });
  if (video.value) {
    video.value.srcObject = stream;
  }
}

async function detectFace() {
  if (!video.value || !canvas.value) return;

  const displaySize = {
    width: video.value.videoWidth,
    height: video.value.videoHeight,
  };

  faceapi.matchDimensions(canvas.value, displaySize);

  setInterval(async () => {
    if (!video.value || !canvas.value) return;

    const detections = await faceapi.detectAllFaces(
      video.value,
      new faceapi.TinyFaceDetectorOptions({
        inputSize: 224,
        scoreThreshold: 0.7,
      }),
    );

    const resized = faceapi.resizeResults(detections, displaySize);
    const ctx = canvas.value.getContext("2d");
    if (!ctx) return;
    ctx.clearRect(0, 0, canvas.value.width, canvas.value.height);

    faceapi.draw.drawDetections(canvas.value, resized);

    // Auto-verify when face is detected and meets quality criteria
    const now = Date.now();
    const canVerify =
      !isVerifying.value &&
      now - lastVerificationTime.value > VERIFICATION_COOLDOWN;

    if (detections.length > 0 && canVerify) {
      const detection = detections[0]; // Use first detected face
      if (!detection) return;

      const box = detection.box;

      // Check if face is well-positioned and sized
      const videoWidth = video.value.videoWidth;
      const videoHeight = video.value.videoHeight;

      // Face should not be too close to edges (at least 10% margin)
      const margin = 0.1;
      const isNotCutOff =
        box.x > videoWidth * margin &&
        box.y > videoHeight * margin &&
        box.x + box.width < videoWidth * (1 - margin) &&
        box.y + box.height < videoHeight * (1 - margin);

      // Face should be reasonably sized (at least 20% of frame width)
      const minFaceSize = videoWidth * 0.2;
      const isSizeGood = box.width >= minFaceSize && box.height >= minFaceSize;

      // Face should be reasonably centered (within middle 80% of frame)
      const centerX = box.x + box.width / 2;
      const centerY = box.y + box.height / 2;
      const isCentered =
        centerX > videoWidth * 0.2 &&
        centerX < videoWidth * 0.8 &&
        centerY > videoHeight * 0.2 &&
        centerY < videoHeight * 0.8;

      // High confidence detection
      const hasGoodScore = detection.score > 0.7;

      const isQualityGood =
        isNotCutOff && isSizeGood && isCentered && hasGoodScore;

      if (isQualityGood) {
        // Start counting if this is first quality detection
        if (faceDetectedTime.value === 0) {
          faceDetectedTime.value = now;
        }

        const elapsed = now - faceDetectedTime.value;
        // const remaining = Math.ceil((DETECTION_DELAY - elapsed) / 1000);

        // Verify if face has been stable for DETECTION_DELAY ms
        if (elapsed >= DETECTION_DELAY) {
          statusText.value = "Checked!";
          countdown.value = 0;
          faceDetectedTime.value = 0; // Reset for next detection
          verifyFace();
        } else {
          statusText.value = "Hold still...";
          // countdown.value = remaining;
        }
      } else {
        // Reset counter if quality drops
        faceDetectedTime.value = 0;
        statusText.value = "Position your face";
        countdown.value = 0;
      }
    } else {
      // Reset counter if no face detected
      faceDetectedTime.value = 0;
      statusText.value = "Position your face";
      countdown.value = 0;
    }
  }, 200);
}

async function captureFrame(): Promise<Blob | null> {
  if (!video.value) return null;

  const captureCanvas = document.createElement("canvas");
  captureCanvas.width = video.value.videoWidth;
  captureCanvas.height = video.value.videoHeight;

  const ctx = captureCanvas.getContext("2d");
  if (!ctx) return null;

  ctx.drawImage(video.value, 0, 0);

  // Convert canvas to Blob for multipart/form-data
  return new Promise<Blob | null>((resolve) => {
    captureCanvas.toBlob((blob) => {
      resolve(blob);
    }, "image/jpeg");
  });
}

function formatCheckedOutTime(dateString: string): string {
  const date = new Date(dateString);

  const day = String(date.getDate()).padStart(2, "0");
  const month = date.toLocaleString("en-US", { month: "short" }).toUpperCase();
  const year = date.getFullYear();
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  const seconds = String(date.getSeconds()).padStart(2, "0");

  return `${day}-${month}-${year} ${hours}:${minutes}:${seconds}`;
}

async function verifyFace() {
  // Unlock audio on first interaction
  await unlockAudio();

  isVerifying.value = true;
  lastVerificationTime.value = Date.now();

  const imageBlob = await captureFrame();

  if (!imageBlob) {
    console.error("Gagal menangkap frame");
    toast.error("Gagal menangkap frame");
    await speak("Gagal menangkap frame");
    isVerifying.value = false;
    return { matched: false, error: "Gagal menangkap frame" };
  }

  try {
    const formData = new FormData();
    formData.append("image", imageBlob, "face.jpg");

    const res = await fetch(
      `${import.meta.env.VITE_API_URL}attendances/checkout/face`,
      {
        method: "PUT",
        body: formData,
      },
    );

    const responseData = await res.json();
    console.log("Hasil verifikasi:", responseData);

    if (!res.ok) {
      const errorMessage =
        responseData.error || `HTTP error! status: ${res.status}`;
      toast.error("Check out gagal", {
        description: errorMessage,
      });
      await speak(errorMessage);
      isVerifying.value = false;
      return { matched: false, error: errorMessage };
    }

    if (responseData.success && responseData.data?.matched) {
      const { user, attendance } = responseData.data;
      const formattedTime = formatCheckedOutTime(attendance.checkedOut);

      toast.success(`Terima kasih, ${user.fullName}!`, {
        description: `Check-out pada ${formattedTime}`,
      });
      await speak(`Terima kasih, ${user.fullName}.`);
    } else {
      const errorMessage = responseData.error || "Face not recognized";
      toast.error("Wajah tidak dikenali", {
        description: errorMessage,
      });
      await speak(errorMessage);
    }

    isVerifying.value = false;
    return responseData;
  } catch (error) {
    console.error("Verifikasi wajah gagal:", error);
    toast.error("Verifikasi gagal", {
      description: error instanceof Error ? error.message : String(error),
    });
    await speak(error instanceof Error ? error.message : String(error));
    isVerifying.value = false;
    return { matched: false, error: String(error) };
  }
}

onMounted(async () => {
  await loadModels();
  await startCamera();

  if (video.value && canvas.value) {
    video.value.addEventListener("loadedmetadata", () => {
      if (!video.value || !canvas.value) return;
      canvas.value.width = video.value.videoWidth;
      canvas.value.height = video.value.videoHeight;
      detectFace();
    });
  }
});
</script>

<template>
  <div class="flex flex-col items-center gap-4">
    <Card
      class="bg-red-950/10 w-160 h-120 overflow-hidden p-0 relative flex items-center justify-center border-2 border-red-500 drop-shadow-xl drop-shadow-red-500/50 backdrop-blur-xs"
    >
      <CardContent>
        <div class="relative w-160 h-120">
          <video
            ref="video"
            autoplay
            muted
            playsinline
            width="640"
            height="480"
            class="absolute top-0 left-0"
          />
          <canvas ref="canvas" class="absolute top-0 left-0" />

          <!-- Status overlay -->
          <div
            class="absolute top-4 left-4 bg-red-500/80 px-4 py-2 rounded-lg font-semibold flex items-center gap-2"
          >
            <span class="text-red-900">{{ statusText }}</span>
            <span v-if="countdown > 0" class="text-yellow-400">{{
              countdown
            }}</span>
            <span v-if="statusText === 'Checked!'" class="text-red-900">✓</span>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
