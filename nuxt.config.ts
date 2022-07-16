import { defineNuxtConfig } from "nuxt";
import eslintPlugin from "vite-plugin-eslint";

export default defineNuxtConfig({
  modules: ["@nuxtjs/tailwindcss", "@nuxtjs/color-mode"],
  colorMode: {
    preference: "light", // default theme
    dataValue: "theme", // activate data-theme in <html> tag
  },
  ssr: false,
  vite: {
    plugins: [eslintPlugin()],
  },
});
