/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./src/web/**/*.rs"],
  plugins: [require("@tailwindcss/forms")],
}

