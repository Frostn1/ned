import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import "./index.css";

function App() {
  const [monitors, setMonitors] = useState([]);

  async function fetchMonitors() {
    const monitors = await invoke("fetch_monitors");
    setMonitors(monitors);
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
