import { defineConfig } from 'vite';

export default defineConfig({
    build: {
        lib: {
            entry: './index.ts',  // your entry file
            formats: ['es'],        // Node ESM
        },
        outDir: 'dist',           // output folder
        rollupOptions: {
            external: [/^node:.*$/], // Node built-ins
        },
    },
});