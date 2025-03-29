import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import "./index.css";
import { Monitor } from "./types/monitor";

function App() {
  const [monitors, setMonitors] = useState<Monitor[]>([]);

  async function fetchMonitors() {
    const monitors = await invoke<Monitor[]>("fetch_monitors");
    console.log('sean .. monitors', monitors)
    setMonitors(monitors)
  }

  useEffect(() => {
    fetchMonitors();
  }, []);
  return (
    <div>
      NED
      {JSON.stringify(monitors, null, 2)}
    </div>
  );
}

export default App;
