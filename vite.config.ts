import fs from 'node:fs';
import inertia from '@inertiajs/vite';
import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import { defineConfig, type Plugin } from 'vite';

const DEV_URL = 'http://localhost:5173';

function hotFile(path = 'public/hot'): Plugin {
  return {
    name: 'rust-inertia:hot-file',
    apply: 'serve',
    configureServer(server) {
      server.httpServer?.once('listening', () => {
        fs.mkdirSync('public', { recursive: true });
        fs.writeFileSync(path, DEV_URL);
      });

      const clean = () => fs.rmSync(path, { force: true });
      process.on('exit', clean);
      process.on('SIGINT', () => process.exit());
      process.on('SIGTERM', () => process.exit());
    },
  };
}

export default defineConfig(({ command }) => ({
  base: command === 'build' ? '/build/' : '/',
  publicDir: false,
  plugins: [
    // SSR is intentionally deferred until the client-rendered app is stable.
    inertia({ ssr: false }),
    react(),
    tailwindcss(),
    hotFile(),
  ],
  resolve: {
    alias: { '@': '/resources/js' },
  },
  server: {
    port: 5173,
    strictPort: true,
    origin: DEV_URL,
  },
  build: {
    manifest: true,
    outDir: 'public/build',
    rollupOptions: {
      input: 'resources/js/app.tsx',
    },
  },
}));
