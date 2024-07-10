import { Divider } from "@mantine/core";
import RestoreRoundedIcon from "@mui/icons-material/RestoreRounded";
import SyncRoundedIcon from "@mui/icons-material/SyncRounded";
import IconButton from "@mui/material/IconButton";
import { invoke } from "@tauri-apps/api";
import React from "react";
import "./Actions.scss";

const Actions = () => {
  async function findMonitors() {
    await invoke("find_monitors");
  }
  async function fetchMonitors() {
    const val = await invoke("fetch_monitors");
    console.log("sean val ..", val);
  }
  return (
    <div id={"actions"}>
      {/* <IconButton aria-label="Find Monitors" onClick={findMonitors}>
        <SyncRoundedIcon className={"icon"} />
      </IconButton> */}
      <Divider orientation={"vertical"} flexItem />
      <IconButton
        title={"Fetch Monitors"}
        aria-label="Restore DB"
        onClick={fetchMonitors}
      >
        <RestoreRoundedIcon className={"icon"} />
      </IconButton>
    </div>
  );
};

export default Actions;
