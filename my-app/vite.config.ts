import { resolve } from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { reactClickToComponent } from "vite-plugin-react-click-to-component";
import wasm from "vite-plugin-wasm";

const reloadOnStdin = () => ({
  name: "reload-on-stdin",
  handleHotUpdate({ file, server }) {
    console.log("handleHotUpdate: ", file);
    if (file.endsWith(".html")) {
      // ignore update
      // return [];
    }
  },
  configureServer(server) {
    process.stdin.on("data", (data) => {
      const msgs = new Set(data.toString().split("\n"));
      if (msgs.has("IBAZEL_BUILD_COMPLETED SUCCESS")) {
        console.log("reloading");
      }
    });
  },
});

export default defineConfig({
  plugins: [wasm(), react(), reactClickToComponent(), reloadOnStdin()],
  build: {
    target: "esnext",
    rollupOptions: {
      input: {
        index: resolve(__dirname, "index.html"),
      },
    },
  },
  clearScreen: false,
});
