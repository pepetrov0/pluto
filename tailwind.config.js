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
      "accent-lighter": colors.gray["700"],
      "accent-contrast": colors.white,

      // disabled
      disabled: colors.gray["200"],
      "disabled-darker": colors.gray["400"],

      // danger
      danger: colors.red["200"],
      "danger-darker": colors.red["400"],
      "danger-contrast": colors.red["900"],
    },
    extends: {
      spacing: {
        default: "32rem",
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
