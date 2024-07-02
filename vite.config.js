import { defineConfig } from "vite";

export const port = 3000;
export default defineConfig({
  server: {
    port: port,
  }
});
