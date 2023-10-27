import "./App.css";
import '@mantine/core/styles.css';
import { currentMonitor, availableMonitors } from '@tauri-apps/api/window';

import { MantineProvider } from '@mantine/core';
import { invoke } from "@tauri-apps/api";

function App() {
  const monitor = availableMonitors();
  async function getMonitors() {
    await invoke("displayMonitors")
  }
  return (
    <MantineProvider>
      Welcome to my app1
      <button onClick={getMonitors}>
        Monitors
      </button>
    </MantineProvider>
  );
}

export default App;
