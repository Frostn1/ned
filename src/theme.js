import { grey } from "@mui/material/colors";
import { createTheme } from "@mui/material/styles";
const darkTheme = createTheme({
  fontFamily: "Montserrat, sans-serif",
  text: {
    header: {
      fontSize: 24,
      color: grey["A400"],
    },
  },
  background: "#121212",
  defaultRadius: "md",
  pane: {
    primary: {
      background: "#101010",
    },
  },
});

export { darkTheme };
