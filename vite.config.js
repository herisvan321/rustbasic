import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig(({ mode }) => {
  // Load environment variables from .env file
  // Third argument '' loads all variables without requiring VITE_ prefix
  const env = loadEnv(mode, process.cwd(), '');
  
  const appHost = env.APP_HOST || 'localhost';
  const displayHost = (appHost === '0.0.0.0' || appHost === '') ? 'localhost' : appHost;
  const vitePort = parseInt(env.VITE_PORT || '5173', 10);

  return {
    plugins: [react()],
    resolve: {
      alias: {
        '@': path.resolve(__dirname, './src/resources/js'),
      },
    },
    build: {
      outDir: 'src/dist',
      emptyOutDir: true,
      manifest: true, // Generates manifest.json in the build folder for versioned lookups in production
      rollupOptions: {
        input: 'src/resources/js/main.tsx',
      },
    },
    server: {
      cors: true,
      host: appHost,
      port: vitePort,
      origin: `http://${displayHost}:${vitePort}`,
      hmr: {
        host: displayHost,
      },
    },
  };
});
