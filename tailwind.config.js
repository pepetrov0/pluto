/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./src/web/_components/*.rs"],
  theme: {
    colors: {
      "root-background": colors.stone[100],
      "root-background-dark": colors.stone[800],

      "root-background-contrast": colors.black,
      "root-background-contrast-dark": colors.white,

      "background": colors.white,
      "background-dark": colors.stone[900],

      "background-contrast": colors.black,
      "background-contrast-dark": colors.white,
    },
  },
  plugins: [require("@tailwindcss/forms")],
}

