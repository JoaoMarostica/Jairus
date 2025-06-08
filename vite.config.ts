import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from 'path';
import Components from 'unplugin-vue-components/vite';
import AutoImport from 'unplugin-auto-import/vite';
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
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
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('node_modules')) {
            if (id.includes('naive-ui')) {
              if (id.includes('_internal')) return 'naive-ui_internal'
              if (id.includes('data-table')) return 'naive-ui-data-table'
              if (id.includes('date-picker')) return 'naive-ui-date-picker'
              return 'naive-ui'
            }
            if (id.includes('echarts')) return 'echarts'
            if (id.includes('vue')) return 'vue'
            if (id.includes('xlsx')) return 'xlsx'
            if (id.includes('zrender')) return 'zrender'
            return 'vendor'
          }
        }
      }
    }
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
