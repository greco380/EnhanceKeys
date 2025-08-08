import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],
  
  // Configure server for Tauri
  server: {
    port: 1420,
    strictPort: true,
  },

  // Vite options tailored for Tauri development and building
  // see https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  // see https://tauri.app/v1/api/config#buildconfig.beforebuildcommand
  //
  // If you want to use a custom CLI, uncomment the following line
  // and see https://tauri.app/v1/guides/development/development-guide#custom-cli
  // beforeDevCommand: "npm run dev",
  // beforeBuildCommand: "npm run build",
})); 