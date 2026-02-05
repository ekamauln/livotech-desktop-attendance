import { invoke } from "@tauri-apps/api/core";

let unlocked = false;

export function useTTS() {
  async function unlockAudio() {
    // Must be called once by user interaction
    // Create a silent audio context to unlock audio playback
    try {
      const silentAudio = new Audio(
        "data:audio/wav;base64,UklGRigAAABXQVZFZm10IBIAAAABAAEARKwAAIhYAQACABAAAABkYXRhAgAAAAEA"
      );
      silentAudio.volume = 0;
      await silentAudio.play();
      unlocked = true;
      console.log("Audio unlocked successfully");
    } catch (err) {
      console.warn("Failed to unlock audio:", err);
      unlocked = true; // Try anyway
    }
  }

  async function speak(text: string) {
    if (!text) {
      console.warn("No text provided to speak");
      return;
    }

    if (!unlocked) {
      console.warn("Audio not unlocked yet, attempting to unlock...");
      await unlockAudio();
    }

    try {
      console.log("Calling TTS with text:", text);
      
      // 1️⃣ Call Rust to generate audio and get data URL
      const audioDataUrl = await invoke<string>("tts", { text });
      console.log("TTS generated audio data URL (length):", audioDataUrl.length);

      // 2️⃣ Play audio directly from data URL
      const audio = new Audio(audioDataUrl);
      
      audio.onloadeddata = () => {
        console.log("Audio loaded successfully");
      };
      
      audio.onerror = (err) => {
        console.error("Audio playback error:", err);
      };
      
      audio.onended = () => {
        console.log("Audio playback completed");
      };

      await audio.play();
      console.log("Audio playback started");
    } catch (err) {
      console.error("TTS error:", err);
      throw err;
    }
  }

  return { speak, unlockAudio };
}
