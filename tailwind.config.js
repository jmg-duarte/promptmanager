/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{svelte,js,ts,jsx,tsx}",
    "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
    "./node_modules/flowbite-svelte-icons/**/*.{html,js,svelte,ts}",
  ],
  theme: {
    extend: {
      colors: {
        // flowbite-svelte
        primary: {
          50: '#e3f1ff',
          100: '#bcdcff',
          200: '#91c6ff',
          300: '#65b0ff',
          400: '#479eff',
          500: '#338eff',
          600: '#377ff3',
          700: '#366ddf',
          800: '#355bcc',
          900: '#333aac',
        },
      }
    }
  },
  plugins: [
    require("flowbite/plugin")
  ],
}

