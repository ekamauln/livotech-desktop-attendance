<script setup lang="ts">
import { ref } from "vue";
import { z } from "zod";
import { toast } from "vue-sonner";
import { useTTS } from "@/lib/useTTS";
import { fetch } from "@tauri-apps/plugin-http";
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldSet,
  FieldTitle,
} from "@/components/ui/field";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Spinner } from "@/components/ui/spinner";

// Initialize TTS (Text-to-Speech) - though not used in this component yet
const { speak, unlockAudio } = useTTS();

// Form state
const username = ref("");
const password = ref("");
const isSubmitting = ref(false);

// Form validation errors
const errors = ref<{ username?: string; password?: string }>({});

// Zod schema for validation
const checkInSchema = z.object({
  username: z.string().min(1, "Username is required"),
  password: z.string().min(1, "Password is required"),
});

function formatCheckedInTime(dateString: string): string {
  const date = new Date(dateString);

  const day = String(date.getDate()).padStart(2, "0");
  const month = date.toLocaleString("en-US", { month: "short" }).toUpperCase();
  const year = date.getFullYear();
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  const seconds = String(date.getSeconds()).padStart(2, "0");

  return `${day}-${month}-${year} ${hours}:${minutes}:${seconds}`;
}

async function handleSubmit() {
  // Unlock audio on first interaction
  await unlockAudio();

  // Reset errors
  errors.value = {};

  // Validate form
  const result = checkInSchema.safeParse({
    username: username.value,
    password: password.value,
  });

  if (!result.success) {
    // Map Zod errors to form errors
    result.error.issues.forEach((error) => {
      if (error.path[0]) {
        errors.value[error.path[0] as "username" | "password"] = error.message;
      }
    });
    return;
  }

  isSubmitting.value = true;

  try {
    const res = await fetch(
      `${import.meta.env.VITE_API_URL}attendances/checkin/manual`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          username: username.value,
          password: password.value,
        }),
      },
    );

    const responseData = await res.json();
    console.log("Hasil check-in:", responseData);

    if (!res.ok) {
      const errorMessage =
        responseData.error || `HTTP error! status: ${res.status}`;
      toast.error("Check-in gagal", {
        description: errorMessage,
      });
      await speak(errorMessage);
      isSubmitting.value = false;
      return;
    }

    if (responseData.success && responseData.data?.matched) {
      const { user, attendance } = responseData.data;
      const formattedTime = formatCheckedInTime(attendance.checkedIn);

      toast.success(`Selamat datang, ${user.fullName}!`, {
        description: `Check-in pada ${formattedTime}`,
      });
      await speak(`Selamat datang, ${user.fullName}.`);

      // Clear form on success
      username.value = "";
      password.value = "";
    } else {
      const errorMessage = responseData.error || "Authentication failed";
      toast.error("Check-in gagal", {
        description: errorMessage,
      });
      await speak(errorMessage);
    }

    isSubmitting.value = false;
  } catch (error) {
    console.error("Check-in gagal:", error);
    toast.error("Check-in gagal", {
      description: error instanceof Error ? error.message : String(error),
    });
    await speak(error instanceof Error ? error.message : String(error));
    isSubmitting.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col items-center gap-4">
    <Card
      class="bg-emerald-950/10 w-160 h-110 overflow-hidden flex items-center justify-center border-2 border-emerald-500 drop-shadow-xl drop-shadow-emerald-500/50 backdrop-blur-xs"
    >
      <CardContent class="w-full flex flex-col items-center p-6">
        <form @submit.prevent="handleSubmit">
          <FieldSet>
            <FieldTitle
              class="text-emerald-200 font-semibold text-lg text-center"
            >
              Please enter your credentials below to check in manually for
              today.
            </FieldTitle>
            <FieldGroup>
              <Field>
                <FieldLabel
                  for="username"
                  class="text-emerald-200 font-semibold text-sm"
                >
                  Username
                </FieldLabel>
                <Input
                  id="username"
                  v-model="username"
                  placeholder="Enter your username"
                  :class="[
                    'text-emerald-200',
                    errors.username ? 'border-red-500' : '',
                  ]"
                  autocomplete="off"
                  :disabled="isSubmitting"
                />
                <FieldError v-if="errors.username" class="text-red-500 text-sm">
                  {{ errors.username }}
                </FieldError>
              </Field>
              <Field>
                <FieldLabel
                  for="password"
                  class="text-emerald-200 font-semibold text-sm"
                >
                  Password
                </FieldLabel>
                <Input
                  id="password"
                  v-model="password"
                  type="password"
                  placeholder="Enter your password"
                  :class="[
                    'text-emerald-200',
                    errors.password ? 'border-red-500' : '',
                  ]"
                  :disabled="isSubmitting"
                />
                <FieldError v-if="errors.password" class="text-red-500 text-sm">
                  {{ errors.password }}
                </FieldError>
              </Field>
              <Field>
                <Button
                  type="submit"
                  :disabled="isSubmitting"
                  class="bg-emerald-500 hover:bg-emerald-600 text-white font-semibold text-sm cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <span
                    v-if="isSubmitting"
                    class="flex items-center justify-center gap-2"
                    ><Spinner class="size-5" /> Checking In...
                  </span>
                  <span v-else>Check In</span>
                </Button>
              </Field>
            </FieldGroup>
          </FieldSet>
        </form>
      </CardContent>
    </Card>
  </div>
</template>
