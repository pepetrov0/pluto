/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["templates/**/*.html"],
  theme: {},
  plugins: [require("@tailwindcss/forms")],
};
