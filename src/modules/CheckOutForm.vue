<script setup lang="ts">
import { ref } from "vue";
import { z } from "zod";
import { toast } from "vue-sonner";
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

// Form state
const username = ref("");
const password = ref("");
const isSubmitting = ref(false);

// Form validation errors
const errors = ref<{ username?: string; password?: string }>({});

// Zod schema for validation
const checkOutSchema = z.object({
  username: z.string().min(1, "Username is required"),
  password: z.string().min(1, "Password is required"),
});

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

async function handleSubmit() {
  // Reset errors
  errors.value = {};

  // Validate form
  const result = checkOutSchema.safeParse({
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
      `${import.meta.env.VITE_API_URL}attendances/checkout/manual`,
      {
        method: "PUT",
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
      isSubmitting.value = false;
      return;
    }

    if (responseData.success && responseData.data?.matched) {
      const { user, attendance } = responseData.data;
      const formattedTime = formatCheckedOutTime(attendance.checkedOut);

      toast.success(`Terimah kasih, ${user.fullName}!`, {
        description: `Check out pada ${formattedTime}`,
      });

      // Clear form on success
      username.value = "";
      password.value = "";
    } else {
      const errorMessage = responseData.error || "Authentication failed";
      toast.error("Check-in gagal", {
        description: errorMessage,
      });
    }

    isSubmitting.value = false;
  } catch (error) {
    console.error("Check-in gagal:", error);
    toast.error("Check-in gagal", {
      description: error instanceof Error ? error.message : String(error),
    });
    isSubmitting.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col items-center gap-4">
    <Card
      class="bg-red-950/10 w-160 h-110 overflow-hidden flex items-center justify-center border-2 border-red-500 drop-shadow-xl drop-shadow-red-500/50 backdrop-blur-xs"
    >
      <CardContent class="w-full flex flex-col items-center p-6">
        <form @submit.prevent="handleSubmit">
          <FieldSet>
            <FieldTitle class="text-red-200 font-semibold text-lg text-center">
              Please enter your credentials below to check out manually for
              today.
            </FieldTitle>
            <FieldGroup>
              <Field>
                <FieldLabel
                  for="username"
                  class="text-red-200 font-semibold text-sm"
                >
                  Username
                </FieldLabel>
                <Input
                  id="username"
                  v-model="username"
                  placeholder="Enter your username"
                  :class="[
                    'text-red-200',
                    errors.username ? 'border-red-500' : '',
                  ]"
                  autocomplete="off"
                  :disabled="isSubmitting"
                />
                <FieldError
                  v-if="errors.username"
                  class="text-red-500 text-sm"
                  >{{ errors.username }}</FieldError
                >
              </Field>
              <Field>
                <FieldLabel
                  for="password"
                  class="text-red-200 font-semibold text-sm"
                >
                  Password
                </FieldLabel>
                <Input
                  id="password"
                  v-model="password"
                  type="password"
                  placeholder="Enter your password"
                  :class="[
                    'text-red-200',
                    errors.password ? 'border-red-500' : '',
                  ]"
                  :disabled="isSubmitting"
                />
                <FieldError
                  v-if="errors.password"
                  class="text-red-500 text-sm"
                  >{{ errors.password }}</FieldError
                >
              </Field>
              <Field>
                <Button
                  type="submit"
                  :disabled="isSubmitting"
                  class="bg-red-500 hover:bg-red-600 text-white font-semibold text-sm cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <span
                    v-if="isSubmitting"
                    class="flex items-center justify-center gap-2"
                    ><Spinner class="size-5" /> Checking Out...
                  </span>
                  <span v-else>Check Out</span>
                </Button>
              </Field>
            </FieldGroup>
          </FieldSet>
        </form>
      </CardContent>
    </Card>
  </div>
</template>
