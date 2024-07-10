import MainScreen from "./Components/MainScreen/MainScreen";
import { darkTheme } from "./theme";
import { ThemeProvider } from "@mui/material/styles";
import "./app.scss";
import { MantineProvider } from "@mantine/core";
function App() {
  return (
    <MantineProvider>
      <ThemeProvider theme={darkTheme}>
        <div id={"app"}>
          <MainScreen />
        </div>
      </ThemeProvider>
    </MantineProvider>
  );
}

export default App;
