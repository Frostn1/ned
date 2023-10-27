import "./app.scss";
// import '@mantine/core/styles.css';
import { currentMonitor, availableMonitors } from '@tauri-apps/api/window';

import { MantineProvider, createTheme } from '@mantine/core';
import { invoke } from "@tauri-apps/api";
import {useState} from 'react';

const theme = createTheme({
  fontFamily: 'Montserrat, sans-serif',
  background: 'red',
  defaultRadius: 'md',
});

function App() {
  // const monitor = availableMonitors();
  const [monitors, setMonitors] = useState([]);
  async function getMonitors() {
    setMonitors(await invoke("display_monitors"))
  }
  return (
    <div  id={'app'}>
      Welcome to my app
      <button onClick={getMonitors}>
        Monitors
      </button>
      {JSON.stringify(monitors)}
    </div>
  );
}

export default App;
