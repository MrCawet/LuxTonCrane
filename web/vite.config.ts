import { defineConfig } from 'vitest/config'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [vue(), tailwindcss()],
  server: {
    // En développement, le serveur Rust tourne à part : `cargo run -p serveur`.
    proxy: { '/api': 'http://127.0.0.1:8090' },
  },
  test: {
    environment: 'happy-dom',
    // Les heures affichées dépendent du fuseau : les tests le fixent.
    env: { TZ: 'Europe/Paris' },
  },
})
