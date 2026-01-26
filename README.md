# Livotech Attendance System (Tauri + Vue + TypeScript)

This is an app that run on desktop for Livotech Attendance System built with Tauri, Vue 3, and TypeScript. This app developed with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Quick Start

```bash
# Clone the repo
git clone https://github.com/yourusername/livotech-desktop-attendance.git

# Install dependencies
cd livotech-desktop-attendance
npm install

# Env variables
Create a `.env` file in the root directory and add the following variables:
- VITE_API_URL=Backend Service Domain (ex. http://localhost:8080/api/)

# Run the app in development mode
npm run tauri dev

# Build the app for production
npm run tauri build
```
