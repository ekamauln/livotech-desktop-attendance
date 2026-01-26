<script setup lang="ts">
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
</script>

<template>
  <div class="flex flex-col items-center gap-4">
    <Card
      class="bg-emerald-950/10 w-160 h-110 overflow-hidden flex items-center justify-center border-2 border-emerald-500 drop-shadow-xl drop-shadow-emerald-500/50 backdrop-blur-xs"
    >
      <CardContent class="w-full flex flex-col items-center p-6">
        <FieldSet>
          <FieldTitle
            class="text-emerald-200 font-semibold text-lg text-center"
          >
            Please enter your credentials below to check in manually for today.
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
                placeholder="Enter your username"
                class="text-emerald-200"
                required
              />
              <FieldError class="text-red-500 text-sm"
                >Username is required</FieldError
              >
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
                type="password"
                placeholder="Enter your password"
                class="text-emerald-200"
                required
              />
              <FieldError class="text-red-500 text-sm"
                >Password is required</FieldError
              >
            </Field>
            <Field>
              <Button
                type="submit"
                class="bg-emerald-500 hover:bg-emerald-600 text-white font-semibold text-sm cursor-pointer"
                >Check In</Button
              >
            </Field>
          </FieldGroup>
        </FieldSet>
      </CardContent>
    </Card>
  </div>
</template>
