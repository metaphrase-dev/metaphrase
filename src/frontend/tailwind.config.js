const colors = require("tailwindcss/colors");
const { colors: defaultColors } = require("tailwindcss/defaultTheme");

module.exports = {
  mode: "jit",
  purge: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    colors: {
      ...defaultColors,
      goose: colors.coolGray,
      indigo: {
        ...colors.indigo,
        1000: "#272162",
        1100: "#161145",
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
