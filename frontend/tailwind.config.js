/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,jsx}'],
  theme: {
    extend: {
      colors: {
        paper: '#f3efe6',
        ink: '#1c1915',
        pine: '#1b6b43',
        clay: '#9a3412',
        line: '#e4d9c8',
        card: '#fffdf8',
      },
      fontFamily: {
        sans: ['Segoe UI', 'Helvetica Neue', 'Arial', 'sans-serif'],
        display: ['Iowan Old Style', 'Palatino', 'Georgia', 'serif'],
      },
    },
  },
}
