import { resolve } from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { reactClickToComponent } from "vite-plugin-react-click-to-component";
import wasm from "vite-plugin-wasm";
import * as childProcess from "child_process";

const reloadOnStdin= () => ({
  name: 'reload-on-stdin',
  handleHotUpdate({ file, server }) {
    console.log("handleHotUpdate: ", file);
    if (file.endsWith(".html")) {
      // ignore update
      return [];
    }
  },
  configureServer(server) {
    process.stdin.on('data', data => {
      const msgs = new Set(data.toString().split("\n"));
      // TODO: Move this to bazel rule.
      if (msgs.has("IBAZEL_BUILD_COMPLETED SUCCESS")) {
        console.log("reloading");
        // Run START_SCRIPT --no-exec to completion:
        // childProcess.execSync(process.env["START_SCRIPT"] + " --no-exec", {stdio: 'inherit'});
      }
    });
  }
})

export default defineConfig({
  plugins: [wasm(), react(), reactClickToComponent(), reloadOnStdin()],
  build: {
    rollupOptions: {
      input: {
        index: resolve(__dirname, "index.html"),
      },
    },
  },
});
