import vue from '@vitejs/plugin-vue';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  plugins: [vue(), wasm()],
  base: process.env.GITHUB_PAGES ? '/fingerprint-wasm/' : '/',
  server: {
    host: '0.0.0.0',
    fs: {
      allow: ['../'],
    },
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
});
