import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import preprocess from 'svelte-preprocess';
import { resolve } from 'path';

export default defineConfig({
  plugins: [
    svelte({
      preprocess: preprocess(),
    }),
  ],
  base: './',
  root: resolve(__dirname, 'src/frontend'),
  publicDir: 'assets',
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    outDir: resolve(__dirname, 'dist'),
    emptyOutDir: true,
    assetsDir: 'assets',
    manifest: true,
    rollupOptions: {
      input: resolve(__dirname, 'src/frontend/index.html'),
    }
  },
  server: {
    port: 5173,
    strictPort: true,
  },
  envPrefix: ['VITE_', 'TAURI_']
}); 