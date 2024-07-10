import DisplayContainer from "../DisplayContainer/DisplayContainer";
import Header from "../Header/Header";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Button, useTheme } from "@mui/material";
import "./MainScreen.scss";

const MainScreen = (props) => {
  // const monitor = availableMonitors();
  const [monitors, setMonitors] = useState([]);
  const theme = useTheme();

  async function getMonitors() {
    await invoke("find_monitors");
  }

  useEffect(() => {
    getMonitors();
  }, []);
  return (
    <div id={"main-screen"} style={{ background: theme.background }}>
      <Header />
      <DisplayContainer monitors={monitors} />
      <Button onClick={getMonitors}>Monitors</Button>
      {JSON.stringify(monitors)}
    </div>
  );
};

export default MainScreen;
