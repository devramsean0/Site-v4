/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{astro,js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        'header': '#E4FDE1',
        's1': '#F45B69',
        's2': '#6B2737',
        's3': '#456990',
        'footer': '#114B5F',
      },
      fontFamily: {
        'main': ['DM Sans', 'ui-sans-serif', 'system-ui']
      }
    },
  },
  plugins: [],
}

