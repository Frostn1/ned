import React from 'react'
import './CommandHeader.scss'
import SyncRoundedIcon from '@mui/icons-material/SyncRounded';
import IconButton from '@mui/material/IconButton';
import { invoke } from "@tauri-apps/api";

const CommandHeader = () => {

    async function findMonitors() {
        await invoke("find_monitors")
    }
    return (
        <div id={'command-header'}>
            <IconButton aria-label="Find Monitors" onClick={findMonitors}>
                <SyncRoundedIcon className={'icon'} />
            </IconButton>

        </div>
    )
}

export default CommandHeader