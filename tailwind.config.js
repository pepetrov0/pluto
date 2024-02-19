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
      "accent-lightest": colors.gray["500"],
      "accent-contrast": colors.white,

      // disabled
      disabled: colors.gray["200"],
      "disabled-darker": colors.gray["400"],

      // danger
      danger: colors.red["200"],
      "danger-darker": colors.red["400"],
      "danger-contrast": colors.red["900"],

      // warn
      warn: colors.amber["200"],
      "warn-darker": colors.amber["400"],
      "warn-contrast": colors.amber["900"],

      // success
      success: colors.green["200"],
      "success-darker": colors.green["400"],
      "success-contrast": colors.green["900"],
    },
    extend: {
      spacing: {
        default: "32rem",
      },
      keyframes: {
        wiggle: {
          "0%, 10%, 30%, 40%": { transform: "rotate(0deg)" },
          "5%, 25%": { transform: "rotate(-15deg)" },
          "15%, 35%": { transform: "rotate(15deg)" },
        },
      },
      animation: {
        wiggle: "wiggle 1s ease-in-out infinite",
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
