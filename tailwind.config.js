/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["www/templates/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
