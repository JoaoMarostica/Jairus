import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from 'path';
import Components from 'unplugin-vue-components/vite';
import AutoImport from 'unplugin-auto-import/vite';
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  base: './',
  plugins: [vue(), 
    AutoImport({
      resolvers: [NaiveUiResolver()],
    }),
    Components({
      resolvers: [NaiveUiResolver()],
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    }
  },

  clearScreen: false,
  build: {
    target: 'esnext',
    minify: 'esbuild',
    chunkSizeWarningLimit: 500,
    outDir: 'dist',
  },
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
