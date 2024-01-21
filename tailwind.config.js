/** @type {import('tailwindcss').Config} */
const colors = require("tailwindcss/colors");

module.exports = {
  content: ["templates/**/*.html"],
  theme: {
    colors: {
      background: colors.gray["100"],

      // default
      default: colors.white,
      "default-darker": colors.gray["200"],
      "default-contrast": colors.gray["900"],

      // accent
      accent: colors.gray["900"],
      "accent-contrast": colors.white,

      // disabled
      disabled: colors.gray["200"],
      "disabled-darker": colors.gray["400"],

      // danger
      danger: colors.red["200"],
      "danger-darker": colors.red["400"],
    },
    extends: {
      spacing: {
        default: "32rem",
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
