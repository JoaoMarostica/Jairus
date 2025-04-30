import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { visualizer } from 'rollup-plugin-visualizer';
import path from 'path';
import Components from 'unplugin-vue-components/vite';
import AutoImport from 'unplugin-auto-import/vite';
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), 
    visualizer({
      filename: './dist/stats.html',
      open: true, // Abre o navegador automaticamente
      gzipSize: true,
      brotliSize: true,
    }),
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
    target: 'esnext', // Compilação para um alvo mais moderno
    minify: 'esbuild', // Usa esbuild para a minificação
    chunkSizeWarningLimit: 500, // Ajusta o tamanho do chunk
    outDir: 'dist', // Garante que os arquivos sejam exportados corretamente
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
      ignored: ["**/src-tauri/**"], // Ignorar a pasta src-tauri
    },
  },
});
