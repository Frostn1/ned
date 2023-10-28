import "./app.scss";
// import '@mantine/core/styles.css';
import { currentMonitor, availableMonitors } from '@tauri-apps/api/window';

import { MantineProvider, createTheme } from '@mantine/core';
import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from 'react';
import Header from "./Components/Header/Header";
import DisplayContainer from "./Components/DisplayContainer/DisplayContainer";
import CommandHeader from "./Components/CommandHeader/CommandHeader";

const theme = createTheme({
  fontFamily: 'Montserrat, sans-serif',
  background: 'red',
  defaultRadius: 'md',
});

function App() {
  // const monitor = availableMonitors();
  const [monitors, setMonitors] = useState([]);

  async function getMonitors() {
    await invoke("find_monitors")
  }

  useEffect(() => {
    getMonitors();
  }, [])

  return (
    <div id={'app'}>
      <Header />
      <div className={'body'}>
        <CommandHeader/>
        <DisplayContainer monitors={monitors}/>
        
      </div>
      {/* <button onClick={getMonitors}> */}
      {/* Monitors */}
      {/* </button> */}
      {/* {JSON.stringify(monitors)} */}
    </div>
  );
}

export default App;
