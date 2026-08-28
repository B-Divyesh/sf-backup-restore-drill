import { resolve } from "node:path";
import { defineConfig } from "vite";

export default defineConfig({
  root: resolve(__dirname),
  build: {
    outDir: resolve(__dirname, "../dist/site"),
    emptyOutDir: true,
    target: "es2022",
    rollupOptions: {
      input: {
        home: resolve(__dirname, "index.html"),
        privacy: resolve(__dirname, "privacy/index.html"),
        terms: resolve(__dirname, "terms/index.html")
      }
    }
  }
});
